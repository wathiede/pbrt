// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::sync::Arc;

use log::error;
use log::info;
use log::warn;

use crate::core::geometry::Vector3f;
use crate::core::light::Light;
use crate::core::medium::Medium;
use crate::core::paramset::ParamSet;
use crate::core::paramset::TextureParams;
use crate::core::parser;
use crate::core::parser::Directive;
use crate::core::pbrt::Float;
use crate::core::pbrt::Options;
use crate::core::spectrum::Spectrum;
use crate::core::texture::Texture;
use crate::core::transform::Matrix4x4;
use crate::core::transform::Transform;
use crate::textures::constant;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parser(parser::Error),
    Unhandled(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<parser::Error> for Error {
    fn from(err: parser::Error) -> Error {
        Error::Parser(err)
    }
}

#[derive(Debug, PartialEq)]
enum APIState {
    Uninitialized,
    OptionsBlock,
    WorldBlock,
}

// API Local Classes
const MAX_TRANSFORMS: usize = 2;
const START_TRANSFORM_BITS: usize = 1;
const END_TRANSFORM_BITS: usize = 2;
const ALL_TRANSFORMS_BITS: usize = (1 << MAX_TRANSFORMS) - 1;

#[derive(Copy, Clone, Debug, Default)]
struct TransformSet {
    t: [Transform; MAX_TRANSFORMS],
}

impl TransformSet {
    fn is_animated(&self) -> bool {
        for i in 0..(MAX_TRANSFORMS - 1) {
            if self.t[i] != self.t[i + 1] {
                return true;
            }
        }
        false
    }
    fn inverse(&self) -> TransformSet {
        let mut t_inv: TransformSet = Default::default();
        for i in 0..MAX_TRANSFORMS {
            t_inv.t[i] = self.t[i].inverse();
        }
        t_inv
    }
}

impl Index<usize> for TransformSet {
    type Output = Transform;
    fn index(&self, idx: usize) -> &Transform {
        debug_assert!(idx < ALL_TRANSFORMS_BITS);
        &self.t[idx]
    }
}

impl IndexMut<usize> for TransformSet {
    fn index_mut(&mut self, idx: usize) -> &mut Transform {
        debug_assert!(idx < ALL_TRANSFORMS_BITS);
        &mut self.t[idx]
    }
}

#[derive(Debug)]
struct RenderOptions {
    transform_start_time: Float,
    transform_end_time: Float,
    filter_name: String,
    filter_params: ParamSet,
    film_name: String,
    film_params: ParamSet,
    sampler_name: String,
    sampler_params: ParamSet,
    accelerator_name: String,
    accelerator_params: ParamSet,
    integrator_name: String,
    integrator_params: ParamSet,
    camera_name: String,
    camera_params: ParamSet,
    camera_to_world: TransformSet,
    named_media: HashMap<String, Medium>,
    lights: Vec<Light>,
    have_scattering_media: bool,
    // TODO(wathiede):
    // std::vector<std::shared_ptr<Primitive>> primitives;
    // std::map<std::string, std::vector<std::shared_ptr<Primitive>>> instances;
    // std::vector<std::shared_ptr<Primitive>> *currentInstance = nullptr;
}

impl RenderOptions {
    fn new() -> RenderOptions {
        RenderOptions {
            transform_start_time: 0.,
            transform_end_time: 0.,
            filter_name: "box".to_owned(),
            filter_params: Default::default(),
            film_name: "image".to_owned(),
            film_params: Default::default(),
            sampler_name: "halton".to_owned(),
            sampler_params: Default::default(),
            accelerator_name: "bvh".to_owned(),
            accelerator_params: Default::default(),
            integrator_name: "path".to_owned(),
            integrator_params: Default::default(),
            camera_name: "perspective".to_owned(),
            camera_params: Default::default(),
            camera_to_world: Default::default(),
            named_media: HashMap::new(),
            lights: Vec::new(),
            have_scattering_media: false,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct GraphicsState {
    current_inside_medium: String,
    current_outside_medium: String,
    // TODO(wathiede):
    // // Graphics State Methods
    // std::shared_ptr<Material> CreateMaterial(const ParamSet &params);
    // MediumInterface CreateMediumInterface();

    // // Graphics State
    float_textures: HashMap<String, Arc<dyn Texture<Float>>>,
    specturm_textures: HashMap<String, Arc<dyn Texture<Spectrum>>>,
    // ParamSet materialParams;
    // std::string material = "matte";
    // std::map<std::string, std::shared_ptr<Material>> namedMaterials;
    // std::string currentNamedMaterial;
    // ParamSet areaLightParams;
    // std::string areaLight;
    // bool reverseOrientation = false;
}

macro_rules! verify_initialized {
    ($pbrt:expr, $func:expr) => {
        if $pbrt.current_api_state == APIState::Uninitialized {
            let msg = format!("init() must be before calling \"{}()\".", $func);
            error!("{}. Ignoring.", msg);
            debug_assert!(false, msg);
            return;
        }
    };
}

#[allow(unused_macros)]
macro_rules! verify_options {
    ($pbrt:expr, $func:expr) => {
        verify_initialized!($pbrt, $func);
        if $pbrt.current_api_state == APIState::WorldBlock {
            let msg = format!(
                "Options cannot be set inside world block; \"{}\" not allowed.",
                $func
            );
            error!("{}. Ignoring.", msg);
            debug_assert!(false, msg);
            return;
        }
    };
}

#[allow(unused_macros)]
macro_rules! verify_world {
    ($pbrt:expr, $func:expr) => {
        verify_initialized!($pbrt, $func);
        if $pbrt.current_api_state == APIState::OptionsBlock {
            let msg = format!(
                "Scene description must be inside world block; \"{}\" not allowed.",
                $func
            );
            error!("{}. Ignoring.", msg);
            debug_assert!(false, msg);
            return;
        }
    };
}

// Pbrt is the top-level global container for all rendering functionality.
#[derive(Debug)]
pub struct Pbrt<'a> {
    opt: &'a Options,
    current_api_state: APIState,
    current_transform: TransformSet,
    active_transform_bits: usize,
    named_coordinate_systems: HashMap<String, TransformSet>,
    render_options: RenderOptions,
    graphics_state: GraphicsState,
    pushed_graphics_states: Vec<GraphicsState>,
    pushed_transforms: Vec<TransformSet>,
    pushed_active_transform_bits: Vec<usize>,
    // TODO(wathiede):
    // static TransformCache transformCache;
}

impl<'a> Pbrt<'a> {
    pub fn new(opt: &'a Options) -> Pbrt<'a> {
        Pbrt {
            opt,
            current_api_state: APIState::Uninitialized,
            current_transform: Default::default(),
            active_transform_bits: ALL_TRANSFORMS_BITS,
            named_coordinate_systems: HashMap::new(),
            render_options: RenderOptions::new(),
            graphics_state: Default::default(),
            pushed_graphics_states: Vec::new(),
            pushed_transforms: Vec::new(),
            pushed_active_transform_bits: Vec::new(),
        }
    }

    // TODO(wathiede): replace Ok() with something that prints stats about the scene render.
    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        let mut f = File::open(path)?;
        let mut buffer = Vec::new();

        // read the whole file
        f.read_to_end(&mut buffer)?;
        let scene = parser::parse_scene(&buffer[..])?;
        info!("Scene {:#?}", &scene);
        for d in scene.directives {
            println!("d: {:?}", &d);
            match d {
                Directive::LookAt(
                    eye_x,
                    eye_y,
                    eye_z,
                    look_x,
                    look_y,
                    look_z,
                    up_x,
                    up_y,
                    up_z,
                ) => self.look_at(
                    [eye_x, eye_y, eye_z],    // eye xyz
                    [look_x, look_y, look_z], // look xyz
                    [up_x, up_y, up_z],       // up xyz
                ),
                Directive::Camera(name, params) => self.camera(name, params),
                Directive::Sampler(name, params) => self.sampler(name, params),
                Directive::Integrator(name, params) => self.integrator(name, params),
                Directive::Film(name, params) => self.film(name, params),
                Directive::WorldBegin => self.world_begin(),
                Directive::WorldEnd => self.world_end(),
                Directive::AttributeBegin => self.attribute_begin(),
                Directive::AttributeEnd => self.attribute_end(),
                Directive::LightSource(_name, _params) => (),
                Directive::Material(_name, _params) => (),
                Directive::Shape(_name, _params) => (),
                Directive::Scale(x, y, z) => self.scale(x, y, z),
                Directive::Rotate(angle, x, y, z) => self.rotate(angle, x, y, z),
                Directive::Translate(x, y, z) => self.translate(x, y, z),
                Directive::Texture(name, kind, texname, params) => {
                    self.texture(&name, &kind, &texname, params)
                }
                Directive::Unhandled(statement) => return Err(Error::Unhandled(statement)),
            }
        }
        Ok(())
    }

    pub fn init(&mut self) {
        if self.current_api_state != APIState::Uninitialized {
            error!("init() has already been called.");
        }
        self.current_api_state = APIState::OptionsBlock;
        self.render_options = RenderOptions::new();
    }

    pub fn cleaup(&mut self) {
        if self.current_api_state == APIState::Uninitialized {
            error!("cleanup() called without init().");
        } else if self.current_api_state == APIState::WorldBlock {
            error!("cleanup() called while inside world block.");
        }
        self.current_api_state = APIState::Uninitialized;
        self.render_options = RenderOptions::new();
    }

    pub fn world_begin(&mut self) {
        verify_options!(self, "pbrt.world_begin");
        self.current_api_state = APIState::WorldBlock;
        for i in 0..MAX_TRANSFORMS {
            self.current_transform[i] = Default::default();
        }
        self.active_transform_bits = ALL_TRANSFORMS_BITS;
        self.named_coordinate_systems
            .insert("world".to_owned(), self.current_transform);
    }

    pub fn world_end(&mut self) {
        verify_world!(self, "pbrt.world_end");
        // TODO(wathiede): call everything
        // // Ensure there are no pushed graphics states
        // while (pushedGraphicsStates.size()) {
        //     Warning("Missing end to pbrtAttributeBegin()");
        //     pushedGraphicsStates.pop_back();
        //     pushedTransforms.pop_back();
        // }
        // while (pushedTransforms.size()) {
        //     Warning("Missing end to pbrtTransformBegin()");
        //     pushedTransforms.pop_back();
        // }

        // // Create scene and render
        // if (PbrtOptions.cat || PbrtOptions.toPly) {
        //     printf("%*sWorldEnd\n", catIndentCount, "");
        // } else {
        //     std::unique_ptr<Integrator> integrator(renderOptions->MakeIntegrator());
        //     std::unique_ptr<Scene> scene(renderOptions->MakeScene());
        //     if (scene && integrator) integrator->Render(*scene);
        // }

        // // Clean up after rendering
        // graphicsState = GraphicsState();
        // transformCache.Clear();
        self.current_api_state = APIState::OptionsBlock;

        // MergeWorkerThreadStats();
        // ReportThreadStats();
        // if (!PbrtOptions.quiet && !PbrtOptions.cat && !PbrtOptions.toPly) {
        //     PrintStats(stdout);
        //     ReportProfilerResults(stdout);
        // }

        // for (int i = 0; i < MaxTransforms; ++i) curTransform[i] = Transform();
        // activeTransformBits = AllTransformsBits;
        // namedCoordinateSystems.erase(namedCoordinateSystems.begin(),
        //                              namedCoordinateSystems.end());
        // ImageTexture<Float, Float>::ClearCache();
        // ImageTexture<RGBSpectrum, Spectrum>::ClearCache();
    }

    pub fn attribute_begin(&mut self) {
        verify_world!(self, "pbrt.attribute_begin");
        self.pushed_graphics_states
            .push(self.graphics_state.clone());
        self.pushed_transforms.push(self.current_transform);
        self.pushed_active_transform_bits
            .push(self.active_transform_bits);
    }

    pub fn attribute_end(&mut self) {
        verify_world!(self, "pbrt.attribute_end");
        if self.pushed_graphics_states.is_empty()
            || self.pushed_transforms.is_empty()
            || self.pushed_active_transform_bits.is_empty()
        {
            error!("Unmatched pbrt.attribute_end() encountered. Ignoring it.");
            return;
        }
        self.graphics_state = self.pushed_graphics_states.pop().unwrap();
        self.current_transform = self.pushed_transforms.pop().unwrap();
        self.active_transform_bits = self.pushed_active_transform_bits.pop().unwrap();
    }

    pub fn transform_begin(&mut self) {
        verify_world!(self, "pbrt.transform_begin");
        self.pushed_transforms.push(self.current_transform);
        self.pushed_active_transform_bits
            .push(self.active_transform_bits);
    }

    pub fn transform_end(&mut self) {
        verify_world!(self, "pbrt.transform_end");
        if self.pushed_transforms.is_empty() || self.pushed_active_transform_bits.is_empty() {
            error!("Unmatched pbrt.tranform_end() encountered. Ignoring it.");
            return;
        }
        self.current_transform = self.pushed_transforms.pop().unwrap();
        self.active_transform_bits = self.pushed_active_transform_bits.pop().unwrap();
    }

    pub fn texture(&mut self, name: &str, kind: &str, texname: &str, params: ParamSet) {
        verify_world!(self, "pbrt.texture");
        info!(
            "Creating texture name {} kind {} texname {} paramset {:?}",
            name, kind, texname, params
        );

        // TODO(wathiede): consider removing clone by using references in TextureParams?
        let tp = TextureParams::new(
            params.clone(),
            params.clone(),
            self.graphics_state.float_textures.clone(),
            self.graphics_state.specturm_textures.clone(),
        );

        match kind {
            "float" => {
                if self.graphics_state.float_textures.contains_key(name) {
                    info!("Float texture '{}' is being redefined", name);
                }
                self.warn_if_animated_transform("pbrt.texture");
                if let Some(ft) = make_float_texture(&texname, &self.current_transform[0], &tp) {
                    self.graphics_state
                        .float_textures
                        .insert(name.to_owned(), Arc::new(ft));
                }
            }
            "color" | "spectrum" => {
                if self.graphics_state.specturm_textures.contains_key(name) {
                    info!("Spectrum texture '{}' is being redefined", name);
                }
                self.warn_if_animated_transform("pbrt.texture");
                if let Some(st) = make_spectrum_texture(&texname, &self.current_transform[0], &tp) {
                    self.graphics_state
                        .specturm_textures
                        .insert(name.to_owned(), Arc::new(st));
                }
            }
            _ => {
                error!("Texture type '{}' is unknown", &kind);
                return;
            }
        }
    }

    pub fn identity(&mut self) {
        verify_initialized!(self, "identity");
        self.for_active_transforms(|ct| *ct = Transform::identity());
    }

    pub fn translate(&mut self, dx: Float, dy: Float, dz: Float) {
        verify_initialized!(self, "translate");
        self.for_active_transforms(|ct| {
            // TODO(wathiede): is it wrong to clone ct? I needed to convert a &mut to a non-mutable
            // type.
            *ct = *ct * Transform::translate(&Vector3f::new(dx, dy, dz))
        });
    }

    pub fn rotate(&mut self, angle: Float, ax: Float, ay: Float, az: Float) {
        verify_initialized!(self, "pbrt.rotate");
        self.for_active_transforms(|ct| {
            *ct = *ct * Transform::rotate(angle, &Vector3f::new(ax, ay, az))
        });
    }

    pub fn look_at(&mut self, eye: [Float; 3], look: [Float; 3], up: [Float; 3]) {
        verify_initialized!(self, "pbrt.look_at");
        info!("eye: {:?} look: {:?} up: {:?}", eye, look, up);
    }

    pub fn scale(&mut self, sx: Float, sy: Float, sz: Float) {
        verify_initialized!(self, "pbrt.scale");
        self.for_active_transforms(|ct| *ct = *ct * Transform::scale(sx, sy, sz));
    }

    pub fn concat_transform(&mut self, transform: [Float; 16]) {
        verify_initialized!(self, "pbrt.concat_transform");
        self.for_active_transforms(|ct| {
            let t = transform;
            *ct = *ct
                * Transform::from(Matrix4x4::new(
                    [t[0], t[1], t[2], t[3]],
                    [t[4], t[5], t[6], t[7]],
                    [t[8], t[9], t[10], t[11]],
                    [t[12], t[13], t[14], t[15]],
                ))
        });
    }

    pub fn transform(&mut self, transform: [Float; 16]) {
        verify_initialized!(self, "pbrt.transform");
        self.for_active_transforms(|ct| {
            let t = transform;
            *ct = Transform::from(Matrix4x4::new(
                [t[0], t[1], t[2], t[3]],
                [t[4], t[5], t[6], t[7]],
                [t[8], t[9], t[10], t[11]],
                [t[12], t[13], t[14], t[15]],
            ))
        });
    }

    pub fn coordinate_system(&mut self, name: &str) {
        verify_initialized!(self, "pbrt.coordinate_system");
        self.named_coordinate_systems
            .insert(name.to_string(), self.current_transform);
    }

    pub fn coordinate_system_transform(&mut self, name: &str) {
        verify_initialized!(self, "pbrt.coordinate_system_transform");
        match self.named_coordinate_systems.get(name) {
            Some(t) => self.current_transform = *t,
            None => warn!("Couldn’t find named coordinate system \"{}\"", name),
        }
    }

    pub fn active_transform_all(&mut self) {
        self.active_transform_bits = ALL_TRANSFORMS_BITS;
    }

    pub fn active_transform_end_time(&mut self) {
        self.active_transform_bits = END_TRANSFORM_BITS;
    }

    pub fn active_transform_start_time(&mut self) {
        self.active_transform_bits = START_TRANSFORM_BITS;
    }

    pub fn transform_times(&mut self, start: Float, end: Float) {
        verify_options!(self, "pbrt.tranform_times");
        self.render_options.transform_start_time = start;
        self.render_options.transform_end_time = end;
    }

    pub fn pixel_filter(&mut self, name: String, params: ParamSet) {
        verify_options!(self, "pbrt.pixel_filter");
        self.render_options.filter_name = name;
        self.render_options.filter_params = params;
    }

    pub fn film(&mut self, name: String, params: ParamSet) {
        verify_options!(self, "pbrt.film");
        self.render_options.film_name = name;
        self.render_options.film_params = params;
    }

    pub fn sampler(&mut self, name: String, params: ParamSet) {
        verify_options!(self, "pbrt.sampler");
        self.render_options.sampler_name = name;
        self.render_options.sampler_params = params;
    }

    pub fn accelerator(&mut self, name: String, params: ParamSet) {
        verify_options!(self, "pbrt.accelerator");
        self.render_options.accelerator_name = name;
        self.render_options.accelerator_params = params;
    }

    pub fn integrator(&mut self, name: String, params: ParamSet) {
        verify_options!(self, "pbrt.integrator");
        self.render_options.integrator_name = name;
        self.render_options.integrator_params = params;
    }

    pub fn camera(&mut self, name: String, params: ParamSet) {
        verify_options!(self, "pbrt.camera");
        self.render_options.camera_name = name;
        self.render_options.camera_params = params;
        self.render_options.camera_to_world = self.current_transform.inverse();
        self.named_coordinate_systems
            .insert("camera".to_owned(), self.render_options.camera_to_world);
    }

    pub fn make_named_medium(&mut self, name: String, params: &mut ParamSet) {
        verify_initialized!(self, "pbrt.make_named_medium");
        self.warn_if_animated_transform("pbrt.make_named_medium");
        let kind = params.find_one_string("type", "");
        let medium = make_medium(&kind, params, self.current_transform[0]);
        self.render_options.named_media.insert(name, medium);
    }

    pub fn medium_interface(&mut self, inside_name: &str, outside_name: &str) {
        verify_initialized!(self, "pbrt.medium_interface");
        self.graphics_state.current_inside_medium = inside_name.into();
        self.graphics_state.current_outside_medium = outside_name.into();
        self.render_options.have_scattering_media = true;
    }

    fn for_active_transforms<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Transform),
    {
        for i in 0..MAX_TRANSFORMS {
            if self.active_transform_bits & (1 << i) > 0 {
                f(&mut self.current_transform[i])
            }
        }
    }

    fn warn_if_animated_transform(&self, name: &str) {
        if self.current_transform.is_animated() {
            warn!(
                "Animated transformations set; ignoring for \"{}\" and using the start transform only",                name);
        }
    }
}

fn make_float_texture(
    name: &str,
    tex2world: &Transform,
    tp: &TextureParams,
) -> Option<Box<dyn Texture<Float>>> {
    match name {
        "constant" => Some(Box::new(constant::create_constant_float_texture(
            tex2world, tp,
        ))),
        "scale" | "mix" | "bilerp" | "imagemap" | "uv" | "checkerboard" | "dots" | "fbm"
        | "wrinkled" | "marble" | "windy" => {
            unimplemented!("Float texture type '{}' not implemented", name);
        }
        _ => {
            warn!("Float texture '{}' is unknown", name);
            None
        }
    }
}

fn make_spectrum_texture(
    name: &str,
    tex2world: &Transform,
    tp: &TextureParams,
) -> Option<Box<dyn Texture<Spectrum>>> {
    match name {
        "constant" => Some(Box::new(constant::create_constant_spectrum_texture(
            tex2world, tp,
        ))),
        "scale" | "mix" | "bilerp" | "imagemap" | "uv" | "checkerboard" | "dots" | "fbm"
        | "wrinkled" | "marble" | "windy" => {
            unimplemented!("Spectrum texture type '{}' not implemented", name);
        }
        _ => {
            warn!("Spectrum texture '{}' is unknown", name);
            None
        }
    }
}

fn make_medium(_name: &str, _params: &mut ParamSet, _medium2world: Transform) -> Medium {
    unimplemented!("make_medium");
}

#[cfg(test)]
mod tests {
    use super::*;
    fn new_options() -> Options {
        Options {
            num_threads: 1,
            quick_render: false,
            quiet: false,
            verbose: true,
            image_file: "".to_owned(),
        }
    }

    #[test]
    fn test_transform_set() {
        let ts: TransformSet = Default::default();
        assert!(!ts.is_animated());
    }

    #[test]
    fn test_named_coordinate_systems() {
        let opts = new_options();
        let mut pbrt = Pbrt::new(&opts);
        pbrt.init();
        pbrt.identity();
        pbrt.scale(2., 2., 2.);
        assert_eq!(pbrt.current_transform.t[0].matrix().m[0][0], 2.);

        pbrt.coordinate_system("two".into());
        pbrt.identity();
        pbrt.scale(3., 3., 3.);
        assert_eq!(pbrt.current_transform.t[0].matrix().m[0][0], 3.);

        pbrt.coordinate_system_transform("two".into());
        assert_eq!(pbrt.current_transform.t[0].matrix().m[0][0], 2.);
    }

    #[test]
    fn test_attribute_begin_end() {
        let opts = new_options();
        let mut pbrt = Pbrt::new(&opts);
        pbrt.init();
        pbrt.world_begin();
        assert_eq!(pbrt.active_transform_bits, ALL_TRANSFORMS_BITS);
        pbrt.attribute_begin();
        pbrt.active_transform_start_time();
        assert_eq!(pbrt.active_transform_bits, START_TRANSFORM_BITS);
        pbrt.attribute_end();
        assert_eq!(pbrt.active_transform_bits, ALL_TRANSFORMS_BITS);
        pbrt.world_end();
    }

    #[test]
    fn test_transform_begin_end() {
        let opts = new_options();
        let mut pbrt = Pbrt::new(&opts);
        pbrt.init();
        pbrt.world_begin();
        assert_eq!(pbrt.active_transform_bits, ALL_TRANSFORMS_BITS);
        pbrt.transform_begin();
        pbrt.active_transform_start_time();
        assert_eq!(pbrt.active_transform_bits, START_TRANSFORM_BITS);
        pbrt.transform_end();
        assert_eq!(pbrt.active_transform_bits, ALL_TRANSFORMS_BITS);
        pbrt.world_end();
    }

    #[test]
    fn test_texture() {
        let opts = new_options();
        let mut pbrt = Pbrt::new(&opts);
        pbrt.init();
        pbrt.world_begin();
        assert_eq!(pbrt.active_transform_bits, ALL_TRANSFORMS_BITS);
        let params = Default::default();
        pbrt.texture("", "", "", params);
        pbrt.world_end();
    }
}
