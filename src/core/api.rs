use std::collections;
use std::fs::File;
use std::io::Read;
use std::io;
use std::ops::{Index, IndexMut};
use std::path::Path;

extern crate nom;

use core::geometry::Vector3f;
use core::light::Light;
use core::medium::Medium;
use core::paramset::ParamSet;
use core::parser;
use core::parser::{OptionsBlock, WorldBlock};
use core::pbrt::{Float, Options};
use core::transform::{Matrix4x4, Transform};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parser(parser::Error),
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
const START_TRANSFORM_BITS: usize = 1 << 0;
const END_TRANSFORM_BITS: usize = 1 << 1;
const ALL_TRANSFORMS_BITS: usize = (1 << MAX_TRANSFORMS) - 1;

#[derive(Copy, Clone, Debug)]
struct TransformSet {
    t: [Transform; MAX_TRANSFORMS],
}

impl TransformSet {
    fn new() -> TransformSet {
        let mut t: [Transform; MAX_TRANSFORMS] = Default::default();
        for i in 0..MAX_TRANSFORMS {
            t[i] = Transform::new();
        }
        TransformSet { t }
    }
    fn is_animated(&self) -> bool {
        for i in 0..(MAX_TRANSFORMS - 1) {
            if self.t[i] != self.t[i + 1] {
                return true;
            }
        }
        false
    }
    fn inverse(&self) -> TransformSet {
        let mut t_inv: TransformSet = TransformSet::new();
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
    named_media: collections::HashMap<String, Medium>,
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
            filter_params: ParamSet::new(),
            film_name: "image".to_owned(),
            film_params: ParamSet::new(),
            sampler_name: "halton".to_owned(),
            sampler_params: ParamSet::new(),
            accelerator_name: "bvh".to_owned(),
            accelerator_params: ParamSet::new(),
            integrator_name: "path".to_owned(),
            integrator_params: ParamSet::new(),
            camera_name: "perspective".to_owned(),
            camera_params: ParamSet::new(),
            camera_to_world: TransformSet::new(),
            named_media: collections::HashMap::new(),
            lights: Vec::new(),
            have_scattering_media: false,
        }
    }
}

#[derive(Debug)]
struct GraphicsState {
    current_inside_medium: String,
    current_outside_medium: String,
    // TODO(wathiede):
    // // Graphics State Methods
    // std::shared_ptr<Material> CreateMaterial(const ParamSet &params);
    // MediumInterface CreateMediumInterface();

    // // Graphics State
    // std::map<std::string, std::shared_ptr<Texture<Float>>> floatTextures;
    // std::map<std::string, std::shared_ptr<Texture<Spectrum>>> spectrumTextures;
    // ParamSet materialParams;
    // std::string material = "matte";
    // std::map<std::string, std::shared_ptr<Material>> namedMaterials;
    // std::string currentNamedMaterial;
    // ParamSet areaLightParams;
    // std::string areaLight;
    // bool reverseOrientation = false;
}

impl GraphicsState {
    fn new() -> GraphicsState {
        GraphicsState {
            current_inside_medium: "".to_owned(),
            current_outside_medium: "".to_owned(),
        }
    }
}
macro_rules! verify_initialized {
    ($pbrt:expr, $func:expr) => (
        if $pbrt.current_api_state == APIState::Uninitialized {
            error!("init() must be before calling \"{}()\".  Ignoring.", $func);
            debug_assert!(false);
            return;
        }
    )
}

#[allow(unused_macros)]
macro_rules! verify_options {
    ($pbrt:expr, $func:expr) => (
        verify_initialized!($pbrt, $func);
        if $pbrt.current_api_state == APIState::WorldBlock {
            error!("Options cannot be set inside world block; \"{}\" not allowed.  Ignoring.",
            $func);
            debug_assert!(false);
            return;
        }
    )
}

#[allow(unused_macros)]
macro_rules! verify_world {
    ($pbrt:expr, $func:expr) => (
        verify_initialized!($pbrt, $func);
        if $pbrt.current_api_state == APIState::OptionsBlock {
            error!("Scene description must be inside world block; \"{}\" not allowed.  Ignoring.",
            $func);
            debug_assert!(false);
            return;
        }
    )
}

// Pbrt is the top-level global container for all rendering functionality.
#[derive(Debug)]
pub struct Pbrt<'a> {
    opt: &'a Options,
    current_api_state: APIState,
    current_transform: TransformSet,
    active_transform_bits: usize,
    named_coordinate_systems: collections::HashMap<String, TransformSet>,
    render_options: RenderOptions,
    graphics_state: GraphicsState,
    // TODO(wathiede):
    // static std::vector<GraphicsState> pushedGraphicsStates;
    // static std::vector<TransformSet> pushedTransforms;
    // static std::vector<uint32_t> pushedActiveTransformBits;
    // static TransformCache transformCache;
}

impl<'a> Pbrt<'a> {
    pub fn new(opt: &'a Options) -> Pbrt<'a> {
        Pbrt {
            opt,
            current_api_state: APIState::Uninitialized,
            current_transform: TransformSet::new(),
            active_transform_bits: ALL_TRANSFORMS_BITS,
            named_coordinate_systems: collections::HashMap::new(),
            render_options: RenderOptions::new(),
            graphics_state: GraphicsState::new(),
        }
    }

    pub fn parse_world_objects(&mut self, world_objects: Vec<WorldBlock>) {
        for wo in world_objects {
            println!("wo: {:?}", &wo);
            match wo {
                WorldBlock::Attribute(wb) => {
                    self.attribute_begin();
                    self.parse_world_objects(wb);
                    self.attribute_end();
                }
                WorldBlock::LightSource(_name, _ps) => (),
                WorldBlock::Material(_name, _ps) => (),
                WorldBlock::Shape(_name, _ps) => (),
                WorldBlock::Translate(x, y, z) => self.translate(x, y, z),
                WorldBlock::Texture(_name, _kind, _class, _ps) => (),
            }
        }
    }

    // TODO(wathiede): replace Ok() with something that prints stats about the scene render.
    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        let mut f = File::open(path)?;
        let mut buffer = Vec::new();

        // read the whole file
        f.read_to_end(&mut buffer)?;
        let scene = parser::parse_scene(&buffer[..])?;
        for o in scene.options {
            println!("o: {:?}", &o);
            match o {
                OptionsBlock::LookAt(
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
                OptionsBlock::Camera(name, ps) => self.camera(name, ps),
                OptionsBlock::Sampler(name, ps) => self.sampler(name, ps),
                OptionsBlock::Integrator(name, ps) => self.integrator(name, ps),
                OptionsBlock::Film(name, ps) => self.film(name, ps),
            }
        }
        self.parse_world_objects(scene.world_objects);
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

    pub fn attribute_begin(&mut self) {
        verify_world!(self, "pbrt.attribute_begin");
        //self.pushed_graphics_states.push_back(graphics_state);
        //self.pushed_transforms.push_back(cur_transform);
        //self.pushed_active_transform_bits.push_back(active_transform_bits);
    }

    pub fn attribute_end(&mut self) {
        verify_world!(self, "pbrt.attribute_end");
        // if (!pushedGraphicsStates.size()) {
        //     Error(
        //         "Unmatched pbrtAttributeEnd() encountered. "
        //         "Ignoring it.");
        //     return;
        // }
        // graphicsState = pushedGraphicsStates.back();
        // pushedGraphicsStates.pop_back();
        // curTransform = pushedTransforms.back();
        // pushedTransforms.pop_back();
        // activeTransformBits = pushedActiveTransformBits.back();
        // pushedActiveTransformBits.pop_back();
    }

    pub fn identity(&mut self) {
        verify_initialized!(self, "identity");
        self.for_active_transforms(|ct| *ct = Transform::new());
    }

    pub fn translate(&mut self, dx: Float, dy: Float, dz: Float) {
        verify_initialized!(self, "translate");
        self.for_active_transforms(|ct| {
            // TODO(wathiede): is it wrong to clone ct? I needed to convert a &mut to a non-mutable
            // type.
            *ct = ct.clone() * Transform::translate(Vector3f::new(dx, dy, dz))
        });
    }

    pub fn rotate(&mut self, angle: Float, ax: Float, ay: Float, az: Float) {
        verify_initialized!(self, "pbrt.rotate");
        self.for_active_transforms(|ct| {
            *ct = ct.clone() * Transform::rotate(angle, Vector3f::new(ax, ay, az))
        });
    }

    pub fn look_at(&mut self, eye: [Float; 3], look: [Float; 3], up: [Float; 3]) {
        verify_initialized!(self, "pbrt.look_at");
        info!("eye: {:?} look: {:?} up: {:?}", eye, look, up);
    }

    pub fn scale(&mut self, sx: Float, sy: Float, sz: Float) {
        verify_initialized!(self, "pbrt.scale");
        self.for_active_transforms(|ct| *ct = ct.clone() * Transform::scale(sx, sy, sz));
    }

    pub fn concat_transform(&mut self, transform: [Float; 16]) {
        verify_initialized!(self, "pbrt.concat_transform");
        self.for_active_transforms(|ct| {
            let t = transform;
            *ct = ct.clone()
                * Transform::new_with_matrix(Matrix4x4::new_with_values(
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
            *ct = Transform::new_with_matrix(Matrix4x4::new_with_values(
                [t[0], t[1], t[2], t[3]],
                [t[4], t[5], t[6], t[7]],
                [t[8], t[9], t[10], t[11]],
                [t[12], t[13], t[14], t[15]],
            ))
        });
    }

    pub fn coordinate_system(&mut self, name: String) {
        verify_initialized!(self, "pbrt.coordinate_system");
        self.named_coordinate_systems
            .insert(name, self.current_transform);
    }

    pub fn coordinate_system_transform(&mut self, name: String) {
        verify_initialized!(self, "pbrt.coordinate_system_transform");
        match self.named_coordinate_systems.get(&name) {
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

fn make_medium(_name: &str, _params: &mut ParamSet, _medium2world: Transform) -> Medium {
    unimplemented!("make_medium");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_set() {
        let ts: TransformSet = TransformSet::new();
        assert!(!ts.is_animated());
    }

    #[test]
    fn test_named_coordinate_systems() {
        let opts = Options {
            num_threads: 1,
            quick_render: false,
            quiet: false,
            verbose: true,
            image_file: "".to_owned(),
        };
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
}
