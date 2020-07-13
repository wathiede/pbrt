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

//! Top-level control of the pbrt state machine.  The parser will construct a `PbrtAPI` and call
//! member functions as it interprets a scene file.

use std::collections::HashMap;
use std::io;
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::process::exit;
use std::sync::Arc;

use log::{error, info, warn};
use thiserror::Error;

use crate::core::filter::Filter;
use crate::core::light::Light;
use crate::core::medium::Medium;
use crate::core::paramset::ParamSet;
use crate::core::paramset::TextureParams;
use crate::core::parser;
use crate::core::spectrum::Spectrum;
use crate::core::texture::Texture;
use crate::core::transform::Transform;
use crate::filters::boxfilter::BoxFilter;
use crate::textures::constant;
use crate::Degree;
use crate::Float;
use crate::Options;

/// Common error type for all public methods in the `api` crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Wrapper for `std::io::Error`s
    #[error("IO error")]
    Io(#[from] io::Error),
    /// Wrapper for errors coming from [parser].
    ///
    /// [parser]: crate::core::parser
    #[error("parse error")]
    Parser(#[from] parser::Error),
    /// Unknown errors, wraps a string for human consumption.
    #[error("unknown error")]
    Unhandled(String),
}

/// Trait describing all the global state machine modifiers that can be called while parsing a
/// scene.  There is a concrete implementation in [PbrtAPI] that implements the rendered as
/// described in the book.  All of the methods have stub implementations that call
/// `unimplemented!()`, this allows test implementations to be created and passed to the parser
/// with only the methods of interest defined.
pub trait API {
    /// Sets the renderer's accelerator settings to `name` & `params`.
    fn accelerator(&mut self, _name: String, _params: ParamSet) {
        unimplemented!()
    }
    /// Sets the active transform bits to `ALL_TRANSFORMS_BITS`.
    fn active_transform_all(&mut self) {
        unimplemented!()
    }
    /// Sets the active transform bits to `END_TRANSFORMS_BITS`.
    fn active_transform_end_time(&mut self) {
        unimplemented!()
    }
    /// Sets the active transform bits to `START_TRANSFORMS_BITS`.
    fn active_transform_start_time(&mut self) {
        unimplemented!()
    }
    /// Called when parser sees a `AttributeBegin` keyword
    fn attribute_begin(&mut self) {
        unimplemented!()
    }
    /// Called when parser sees a `AttributeEnd` keyword
    fn attribute_end(&mut self) {
        unimplemented!()
    }
    /// Sets the renderer's camera settings to `name` & `params`.
    fn camera(&mut self, _name: String, _params: ParamSet) {
        unimplemented!()
    }
    /// Reset the internal state of self.
    fn cleanup(&mut self) {
        unimplemented!()
    }
    /// Multiples the current transform matrix by `transform`.
    fn concat_transform(&mut self, _transform: [Float; 16]) {
        unimplemented!()
    }
    /// Creates a new coordinate system assigning `name` the current tranform matrix.
    fn coordinate_system(&mut self, _name: &str) {
        unimplemented!()
    }
    /// Sets the current transform matrix to the one stored under `name`.
    fn coordinate_system_transform(&mut self, _name: &str) {
        unimplemented!()
    }
    /// Sets the renderer's film settings to `name` & `params`.
    fn film(&mut self, _name: String, _params: ParamSet) {
        unimplemented!()
    }
    /// Sets the currently active transform matrix by the given values.
    fn identity(&mut self) {
        unimplemented!()
    }
    /// Moves the internal statemachine from `APIState::Uninitialized` to `APIState::OptionsBlock`.
    /// This function must be called before most of the API will work.
    fn init(&mut self) {
        unimplemented!()
    }
    /// Sets the renderer's integrator settings to `name` & `params`.
    fn integrator(&mut self, _name: String, _params: ParamSet) {
        unimplemented!()
    }
    /// Sets the current transforms to look at the given directions.
    fn look_at(&mut self, _eye: [Float; 3], _look: [Float; 3], _up: [Float; 3]) {
        unimplemented!()
    }
    /// Creates a medium with the given `params` and stores it as a named media under `name`.
    fn make_named_medium(&mut self, _name: String, _params: &mut ParamSet) {
        unimplemented!()
    }
    /// Specifies the current inside and outside media by the names given.  Cameras and lights
    /// without geometry ignore the `inside_name`.
    fn medium_interface(&mut self, _inside_name: &str, _outside_name: &str) {
        unimplemented!()
    }
    /// Parse a scene file at `path` on the file-system.  This will parse the contents of the file
    /// generating an inmemory representation of the scene, and trigger the rendering and output of
    /// the image.
    fn parse_file<P: AsRef<Path>>(&mut self, _path: P) -> Result<(), Error> {
        unimplemented!()
    }
    /// Parse a scene file represented as text stored in `data`.  This will parse the contents of
    /// data generating an inmemory representation of the scene, and trigger the rendering and
    /// output of
    /// the image.
    fn parse_string(&mut self, _data: &[u8]) -> Result<(), Error> {
        unimplemented!()
    }
    /// Sets the renderer's filter settings to `name` & `params`.
    fn pixel_filter(&mut self, _name: String, _params: ParamSet) {
        unimplemented!()
    }
    /// Rotates the currently active transform matrix by the given values.
    fn rotate(&mut self, _angle: Degree, _ax: Float, _ay: Float, _az: Float) {
        unimplemented!()
    }
    /// Sets the renderer's sampler settings to `name` & `params`.
    fn sampler(&mut self, _name: String, _params: ParamSet) {
        unimplemented!()
    }
    /// Scales the currently active transform matrix by the given values.
    fn scale(&mut self, _sx: Float, _sy: Float, _sz: Float) {
        unimplemented!()
    }
    /// Called when the parser sees a `Texture` line.
    fn texture(&mut self, _name: &str, _kind: &str, _texname: &str, _params: ParamSet) {
        unimplemented!()
    }
    /// Called when parser sees a `TransformBegin` keyword
    fn transform_begin(&mut self) {
        unimplemented!()
    }
    /// Called when parser sees a `TransformEnd` keyword
    fn transform_end(&mut self) {
        unimplemented!()
    }
    /// Sets the current transform matrix to `transform`.
    fn transform(&mut self, _transform: [Float; 16]) {
        unimplemented!()
    }
    /// Sets the start/end times for the transform matrix to `start` & `end`.
    fn transform_times(&mut self, _start: Float, _end: Float) {
        unimplemented!()
    }
    /// Translates the currently active transform matrix by the given values.
    fn translate(&mut self, _dx: Float, _dy: Float, _dz: Float) {
        unimplemented!()
    }
    /// Called when parser sees a `WorldBegin` keyword
    fn world_begin(&mut self) {
        unimplemented!()
    }
    /// Called when parser sees a `WorldEnd` keyword
    fn world_end(&mut self) {
        unimplemented!()
    }
}

/// State machine for the API.
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
    /* TODO(wathiede):
     * std::vector<std::shared_ptr<Primitive>> primitives;
     * std::map<std::string, std::vector<std::shared_ptr<Primitive>>> instances;
     * std::vector<std::shared_ptr<Primitive>> *currentInstance = nullptr; */
}

impl Default for RenderOptions {
    fn default() -> RenderOptions {
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
    /* ParamSet materialParams;
     * std::string material = "matte";
     * std::map<std::string, std::shared_ptr<Material>> namedMaterials;
     * std::string currentNamedMaterial;
     * ParamSet areaLightParams;
     * std::string areaLight;
     * bool reverseOrientation = false; */
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

/// PbrtAPI is the top-level global container for all rendering functionality.
#[derive(Debug)]
pub struct PbrtAPI {
    opt: Options,
    current_api_state: APIState,
    current_transform: TransformSet,
    active_transform_bits: usize,
    named_coordinate_systems: HashMap<String, TransformSet>,
    render_options: RenderOptions,
    graphics_state: GraphicsState,
    pushed_graphics_states: Vec<GraphicsState>,
    pushed_transforms: Vec<TransformSet>,
    pushed_active_transform_bits: Vec<usize>,
    /* TODO(wathiede):
     * static TransformCache transformCache; */
}

impl From<Options> for PbrtAPI {
    /// Creates a `PbrtAPI` from the given options.
    fn from(opt: Options) -> Self {
        PbrtAPI {
            opt,
            current_api_state: APIState::Uninitialized,
            current_transform: Default::default(),
            active_transform_bits: ALL_TRANSFORMS_BITS,
            named_coordinate_systems: HashMap::new(),
            render_options: Default::default(),
            graphics_state: Default::default(),
            pushed_graphics_states: Vec::new(),
            pushed_transforms: Vec::new(),
            pushed_active_transform_bits: Vec::new(),
        }
    }
}

impl PbrtAPI {
    /// Parse a scene file at `path` on the file-system.  This will parse the contents of the file
    /// generating an inmemory representation of the scene, and trigger the rendering and output of
    /// the image.
    pub fn parse_file<P: AsRef<Path>>(&mut self, _path: P) -> Result<(), Error> {
        /*
        let f = File::open(&path)?;
        let mmap = unsafe {
            MmapOptions::new()
                .map(&f)
                .with_context(|| format!("failed to mmap {}", path.display()))?
        };
        */
        unimplemented!();
    }

    /// Verifies all the active transforms are equivalent to `t`.
    ///
    /// # Note
    /// This method isn't part of the API described by pbrt. It exists to make rustdoc
    /// implementations easier.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::api::PbrtAPI;
    /// use pbrt::core::transform::Matrix4x4;
    ///
    /// let mut pbrt: PbrtAPI = Default::default();
    ///
    /// pbrt.init();
    /// pbrt.identity();
    /// pbrt.assert_transforms(Matrix4x4::identity());
    /// ```
    pub fn assert_transforms<T: Into<Transform>>(&self, t: T) {
        let t = t.into();
        self.for_active_transforms(|ct| assert_eq!(ct, &t));
    }

    /// Moves the internal statemachine from `APIState::Uninitialized` to `APIState::OptionsBlock`.
    /// This function must be called before most of the API will work.
    pub fn init(&mut self) {
        if self.current_api_state != APIState::Uninitialized {
            error!("init() has already been called.");
        }
        self.current_api_state = APIState::OptionsBlock;
        self.render_options = Default::default();
    }

    /// Reset the internal state of self.
    pub fn cleanup(&mut self) {
        if self.current_api_state == APIState::Uninitialized {
            error!("cleanup() called without init().");
        } else if self.current_api_state == APIState::WorldBlock {
            error!("cleanup() called while inside world block.");
        }
        self.current_api_state = APIState::Uninitialized;
        self.render_options = Default::default();
    }

    /// Called when parser sees a `WorldBegin` keyword
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

    /// Called when parser sees a `WorldEnd` keyword
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

    /// Called when parser sees a `AttributeBegin` keyword
    pub fn attribute_begin(&mut self) {
        verify_world!(self, "pbrt.attribute_begin");
        self.pushed_graphics_states
            .push(self.graphics_state.clone());
        self.pushed_transforms.push(self.current_transform);
        self.pushed_active_transform_bits
            .push(self.active_transform_bits);
    }

    /// Called when parser sees a `AttributeEnd` keyword
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

    /// Called when parser sees a `TransformBegin` keyword
    pub fn transform_begin(&mut self) {
        verify_world!(self, "pbrt.transform_begin");
        self.pushed_transforms.push(self.current_transform);
        self.pushed_active_transform_bits
            .push(self.active_transform_bits);
    }

    /// Called when parser sees a `TransformEnd` keyword
    pub fn transform_end(&mut self) {
        verify_world!(self, "pbrt.transform_end");
        if self.pushed_transforms.is_empty() || self.pushed_active_transform_bits.is_empty() {
            error!("Unmatched pbrt.tranform_end() encountered. Ignoring it.");
            return;
        }
        self.current_transform = self.pushed_transforms.pop().unwrap();
        self.active_transform_bits = self.pushed_active_transform_bits.pop().unwrap();
    }

    /// Called when the parser sees a `Texture` line.
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

    /// Sets the currently active transform matrix by the given values.
    /// # Examples
    /// ```
    /// use pbrt::core::api::PbrtAPI;
    /// use pbrt::core::transform::Matrix4x4;
    ///
    /// let mut pbrt = PbrtAPI::default();
    ///
    /// pbrt.init();
    /// pbrt.identity();
    /// pbrt.assert_transforms(Matrix4x4::new(
    ///     [1., 0., 0., 0.],
    ///     [0., 1., 0., 0.],
    ///     [0., 0., 1., 0.],
    ///     [0., 0., 0., 1.],
    /// ));
    /// ```
    pub fn identity(&mut self) {
        verify_initialized!(self, "identity");
        self.for_active_transforms_mut(|ct| *ct = Transform::identity());
    }

    /// Translates the currently active transform matrix by the given values.
    /// # Examples
    /// ```
    /// use pbrt::core::api::PbrtAPI;
    /// use pbrt::core::transform::Matrix4x4;
    ///
    /// let mut pbrt = PbrtAPI::default();
    ///
    /// pbrt.init();
    /// pbrt.identity();
    /// pbrt.translate(2., 4., 6.);
    /// pbrt.assert_transforms(Matrix4x4::new(
    ///     [1., 0., 0., 2.],
    ///     [0., 1., 0., 4.],
    ///     [0., 0., 1., 6.],
    ///     [0., 0., 0., 1.],
    /// ));
    /// ```
    pub fn translate(&mut self, dx: Float, dy: Float, dz: Float) {
        verify_initialized!(self, "translate");
        self.for_active_transforms_mut(|ct| {
            // TODO(wathiede): is it wrong to clone ct? I needed to convert a &mut to a non-mutable
            // type.
            *ct = *ct * Transform::translate([dx, dy, dz])
        });
    }

    /// Rotates the currently active transform matrix by the given values.
    /// # Examples
    /// ```
    /// use pbrt::core::api::PbrtAPI;
    /// use pbrt::core::transform::Matrix4x4;
    /// use pbrt::Degree;
    ///
    /// let mut pbrt = PbrtAPI::default();
    ///
    /// pbrt.init();
    /// pbrt.identity();
    /// let t_deg = 180.;
    /// pbrt.rotate(t_deg.into(), 1., 0., 0.);
    /// let t_rad = t_deg.to_radians();
    /// let s = t_rad.sin();
    /// let c = t_rad.cos();
    ///
    /// // Rotate about the x-axis.
    /// pbrt.assert_transforms(Matrix4x4::new(
    ///     [1., 0., 0., 0.],
    ///     [0., c, -s, 0.],
    ///     [0., s, c, 0.],
    ///     [0., 0., 0., 1.],
    /// ));
    ///
    /// // Rotate about the y-axis.
    /// pbrt.identity();
    /// pbrt.rotate(t_deg.into(), 0., 1., 0.);
    /// pbrt.assert_transforms(Matrix4x4::new(
    ///     [c, 0., s, 0.],
    ///     [0., 1., 0., 0.],
    ///     [-s, 0., c, 0.],
    ///     [0., 0., 0., 1.],
    /// ));
    ///
    /// // Rotate about the z-axis.
    /// pbrt.identity();
    /// pbrt.rotate(t_deg.into(), 0., 0., 1.);
    /// pbrt.assert_transforms(Matrix4x4::new(
    ///     [c, -s, 0., 0.],
    ///     [s, c, 0., 0.],
    ///     [0., 0., 1., 0.],
    ///     [0., 0., 0., 1.],
    /// ));
    /// ```
    pub fn rotate(&mut self, angle: Degree, ax: Float, ay: Float, az: Float) {
        verify_initialized!(self, "pbrt.rotate");
        self.for_active_transforms_mut(|ct| *ct = *ct * Transform::rotate(angle, [ax, ay, az]));
    }

    /// Sets the current transforms to look at the given directions.
    pub fn look_at(&mut self, eye: [Float; 3], look: [Float; 3], up: [Float; 3]) {
        verify_initialized!(self, "pbrt.look_at");
        info!("eye: {:?} look: {:?} up: {:?}", eye, look, up);
        unimplemented!();
    }

    /// Scales the currently active transform matrix by the given values.
    /// # Examples
    /// ```
    /// use pbrt::core::api::PbrtAPI;
    /// use pbrt::core::transform::Matrix4x4;
    ///
    /// let mut pbrt = PbrtAPI::default();
    ///
    /// pbrt.init();
    /// pbrt.identity();
    /// pbrt.scale(2., 4., 6.);
    /// pbrt.assert_transforms(Matrix4x4::new(
    ///     [2., 0., 0., 0.],
    ///     [0., 4., 0., 0.],
    ///     [0., 0., 6., 0.],
    ///     [0., 0., 0., 1.],
    /// ));
    /// ```
    pub fn scale(&mut self, sx: Float, sy: Float, sz: Float) {
        verify_initialized!(self, "pbrt.scale");
        self.for_active_transforms_mut(|ct| *ct = *ct * Transform::scale(sx, sy, sz));
    }

    /// Multiples the current transform matrix by `transform`.
    pub fn concat_transform(&mut self, transform: [Float; 16]) {
        verify_initialized!(self, "pbrt.concat_transform");
        self.for_active_transforms_mut(|ct| *ct = *ct * Transform::from(transform));
    }

    /// Sets the current transform matrix to `transform`.
    pub fn transform(&mut self, transform: [Float; 16]) {
        verify_initialized!(self, "pbrt.transform");
        self.for_active_transforms_mut(|ct| *ct = Transform::from(transform));
    }

    /// Creates a new coordinate system assigning `name` the current tranform matrix.
    pub fn coordinate_system(&mut self, name: &str) {
        verify_initialized!(self, "pbrt.coordinate_system");
        self.named_coordinate_systems
            .insert(name.to_string(), self.current_transform);
    }

    /// Sets the current transform matrix to the one stored under `name`.
    pub fn coordinate_system_transform(&mut self, name: &str) {
        verify_initialized!(self, "pbrt.coordinate_system_transform");
        match self.named_coordinate_systems.get(name) {
            Some(t) => self.current_transform = *t,
            None => warn!("Couldn’t find named coordinate system \"{}\"", name),
        }
    }

    /// Sets the active transform bits to `ALL_TRANSFORMS_BITS`.
    pub fn active_transform_all(&mut self) {
        self.active_transform_bits = ALL_TRANSFORMS_BITS;
    }

    /// Sets the active transform bits to `END_TRANSFORMS_BITS`.
    pub fn active_transform_end_time(&mut self) {
        self.active_transform_bits = END_TRANSFORM_BITS;
    }

    /// Sets the active transform bits to `START_TRANSFORMS_BITS`.
    pub fn active_transform_start_time(&mut self) {
        self.active_transform_bits = START_TRANSFORM_BITS;
    }

    /// Sets the start/end times for the transform matrix to `start` & `end`.
    pub fn transform_times(&mut self, start: Float, end: Float) {
        verify_options!(self, "pbrt.tranform_times");
        self.render_options.transform_start_time = start;
        self.render_options.transform_end_time = end;
    }

    /// Sets the renderer's filter settings to `name` & `params`.
    pub fn pixel_filter(&mut self, name: String, params: ParamSet) {
        verify_options!(self, "pbrt.pixel_filter");
        self.render_options.filter_name = name;
        self.render_options.filter_params = params;
    }

    /// Sets the renderer's film settings to `name` & `params`.
    pub fn film(&mut self, name: String, params: ParamSet) {
        verify_options!(self, "pbrt.film");
        self.render_options.film_name = name;
        self.render_options.film_params = params;
    }

    /// Sets the renderer's sampler settings to `name` & `params`.
    pub fn sampler(&mut self, name: String, params: ParamSet) {
        verify_options!(self, "pbrt.sampler");
        self.render_options.sampler_name = name;
        self.render_options.sampler_params = params;
    }

    /// Sets the renderer's accelerator settings to `name` & `params`.
    pub fn accelerator(&mut self, name: String, params: ParamSet) {
        verify_options!(self, "pbrt.accelerator");
        self.render_options.accelerator_name = name;
        self.render_options.accelerator_params = params;
    }

    /// Sets the renderer's integrator settings to `name` & `params`.
    pub fn integrator(&mut self, name: String, params: ParamSet) {
        verify_options!(self, "pbrt.integrator");
        self.render_options.integrator_name = name;
        self.render_options.integrator_params = params;
    }

    /// Sets the renderer's camera settings to `name` & `params`.
    pub fn camera(&mut self, name: String, params: ParamSet) {
        verify_options!(self, "pbrt.camera");
        self.render_options.camera_name = name;
        self.render_options.camera_params = params;
        self.render_options.camera_to_world = self.current_transform.inverse();
        self.named_coordinate_systems
            .insert("camera".to_owned(), self.render_options.camera_to_world);
    }

    /// Creates a medium with the given `params` and stores it as a named media under `name`.
    pub fn make_named_medium(&mut self, name: String, params: &mut ParamSet) {
        verify_initialized!(self, "pbrt.make_named_medium");
        self.warn_if_animated_transform("pbrt.make_named_medium");
        let kind = params.find_one_string("type", "".to_string());
        let medium = make_medium(&kind, params, self.current_transform[0]);
        self.render_options.named_media.insert(name, medium);
    }

    /// Specifies the current inside and outside media by the names given.  Cameras and lights
    /// without geometry ignore the `inside_name`.
    pub fn medium_interface(&mut self, inside_name: &str, outside_name: &str) {
        verify_initialized!(self, "pbrt.medium_interface");
        self.graphics_state.current_inside_medium = inside_name.into();
        self.graphics_state.current_outside_medium = outside_name.into();
        self.render_options.have_scattering_media = true;
    }

    fn for_active_transforms<F>(&self, f: F)
    where
        F: Fn(&Transform),
    {
        for i in 0..MAX_TRANSFORMS {
            if self.active_transform_bits & (1 << i) > 0 {
                f(&self.current_transform[i])
            }
        }
    }
    fn for_active_transforms_mut<F>(&mut self, mut f: F)
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

impl Default for PbrtAPI {
    fn default() -> PbrtAPI {
        PbrtAPI::from(Options::default())
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

// TODO(wathiede): remove #[allow(dead_code)] after make_camera is implemented.
#[allow(dead_code)]
fn make_filter(name: &str, param_set: &ParamSet) -> Box<dyn Filter> {
    let filter = match name {
        "box" => Box::new(BoxFilter::create_box_filter(param_set)),
        "gaussian" | "mitchell" | "sinc" | "triangle" => {
            unimplemented!("Filter type '{}' not implemented", name)
        }
        _ => {
            error!("Filter '{}' unknown.", name);
            exit(1);
        }
    };
    param_set.report_unused();
    filter
}

#[cfg(test)]
mod tests {
    use crate::core::paramset::testutils::make_float_param_set;
    use crate::core::transform::Matrix4x4;

    use super::*;

    #[test]
    fn test_transform_set() {
        let ts: TransformSet = Default::default();
        assert!(!ts.is_animated());
    }

    #[test]
    fn test_named_coordinate_systems() {
        let mut pbrt: PbrtAPI = Default::default();
        pbrt.init();
        pbrt.identity();
        pbrt.scale(2., 2., 2.);
        assert_eq!(
            pbrt.current_transform.t[0].matrix(),
            Matrix4x4::new(
                [2., 0., 0., 0.],
                [0., 2., 0., 0.],
                [0., 0., 2., 0.],
                [0., 0., 0., 1.]
            )
        );

        pbrt.coordinate_system("two".into());
        pbrt.identity();
        pbrt.scale(3., 3., 3.);
        assert_eq!(
            pbrt.current_transform.t[0].matrix(),
            Matrix4x4::new(
                [3., 0., 0., 0.],
                [0., 3., 0., 0.],
                [0., 0., 3., 0.],
                [0., 0., 0., 1.]
            )
        );

        pbrt.coordinate_system_transform("two".into());
        assert_eq!(
            pbrt.current_transform.t[0].matrix(),
            Matrix4x4::new(
                [2., 0., 0., 0.],
                [0., 2., 0., 0.],
                [0., 0., 2., 0.],
                [0., 0., 0., 1.]
            )
        );
    }

    #[test]
    fn test_attribute_begin_end() {
        let mut pbrt: PbrtAPI = Default::default();
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
        let mut pbrt: PbrtAPI = Default::default();
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
        let mut pbrt: PbrtAPI = Default::default();
        pbrt.init();
        pbrt.world_begin();
        assert_eq!(pbrt.active_transform_bits, ALL_TRANSFORMS_BITS);
        let params = Default::default();
        pbrt.texture("", "", "", params);
        pbrt.world_end();
    }

    #[test]
    fn test_make_filter() {
        let ps = make_float_param_set("xwidth", vec![1.]);
        let bf = make_filter("box", &ps);
        assert_eq!(bf.radius(), [1., 0.5].into());
        assert_eq!(bf.inv_radius(), [1., 2.].into());
    }
}
