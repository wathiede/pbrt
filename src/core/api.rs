use std::collections;
use std::fs::File;
use std::io::Read;
use std::io;
use std::ops::{Index, IndexMut};
use std::path::Path;

extern crate nom;

use core::pbrt::{Float, Options};
use core::parser;
use core::transform::{Matrix4x4, Transform};
use core::geometry::Vector3f;

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
const ALL_TRANSFORM_BITS: usize = (1 << MAX_TRANSFORMS) - 1;

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
    fn inverse(ts: &TransformSet) -> TransformSet {
        let mut t_inv: TransformSet = Default::default();
        for i in 0..MAX_TRANSFORMS {
            t_inv.t[i] = ts.t[i].inverse();
        }
        t_inv
    }
}

impl Index<usize> for TransformSet {
    type Output = Transform;
    fn index(&self, idx: usize) -> &Transform {
        debug_assert!(idx > 0);
        debug_assert!(idx < ALL_TRANSFORM_BITS);
        &self.t[idx]
    }
}

impl IndexMut<usize> for TransformSet {
    fn index_mut(&mut self, idx: usize) -> &mut Transform {
        debug_assert!(idx > 0);
        debug_assert!(idx < ALL_TRANSFORM_BITS);
        &mut self.t[idx]
    }
}

macro_rules! verify_initialized {
    ($pbrt:expr, $func:expr) => (
        if $pbrt.current_api_state == APIState::Uninitialized {
            error!("init() must be before calling \"{}()\".  Ignoring.", $func);
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
    // TODO(wathiede):
    // static std::unique_ptr<RenderOptions> renderOptions;
    // static GraphicsState graphicsState;
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
            current_transform: Default::default(),
            active_transform_bits: ALL_TRANSFORM_BITS,
            named_coordinate_systems: collections::HashMap::new(),
        }
    }

    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<parser::Scene, Error> {
        let mut f = File::open(path)?;
        let mut buffer = Vec::new();

        // read the whole file
        f.read_to_end(&mut buffer)?;
        let scene = parser::parse_scene(&buffer[..])?;
        Ok(scene)
    }

    pub fn init(&mut self) {
        if self.current_api_state != APIState::Uninitialized {
            error!("init() has already been called.");
        }
        self.current_api_state = APIState::OptionsBlock;
    }

    pub fn cleaup(&mut self) {
        if self.current_api_state == APIState::Uninitialized {
            error!("cleanup() called without init().");
        } else if self.current_api_state == APIState::WorldBlock {
            error!("cleanup() called while inside world block.");
        }
        self.current_api_state = APIState::Uninitialized;
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

    pub fn identity(&mut self) {
        verify_initialized!(self, "identity");
        // TODO(wathiede): default isn't actually the  identity, make it so.
        self.for_active_transforms(|ct| *ct = Default::default());
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
        self.for_active_transforms(|ct| {
            *ct = ct.clone() * Transform::rotate(angle, Vector3f::new(ax, ay, az))
        });
    }

    pub fn look_at(&mut self, eye: [Float; 3], look: [Float; 3], up: [Float; 3]) {
        verify_initialized!(self, "pbrt.look_at");
        info!("eye: {:?} look: {:?} up: {:?}", eye, look, up);
    }
    pub fn scale(&mut self, sx: Float, sy: Float, sz: Float) {
        self.for_active_transforms(|ct| *ct = ct.clone() * Transform::scale(sx, sy, sz));
    }
    pub fn concat_transform(&mut self, transform: [Float; 16]) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_set() {
        let ts: TransformSet = Default::default();
        assert!(!ts.is_animated());
    }

    #[test]
    fn test_named_coordinate_systems() {
        let opts = Options {
            num_threads: None,
            quick_render: false,
            quiet: false,
            verbose: true,
            image_file: None,
            scene_files: vec![],
        };
        let mut pbrt = Pbrt::new(&opts);
        pbrt.transform([
            2., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.
        ]);
        assert_eq!(pbrt.current_transform.t[0].matrix().m[0][0], 2.);

        pbrt.coordinate_system("two".into());
        pbrt.transform([
            3., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.
        ]);
        assert_eq!(pbrt.current_transform.t[0].matrix().m[0][0], 3.);

        pbrt.coordinate_system_transform("two".into());
        assert_eq!(pbrt.current_transform.t[0].matrix().m[0][0], 2.);
    }
}
