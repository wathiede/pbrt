use std::fs::File;
use std::io::Read;
use std::io;
use std::path::Path;

extern crate nom;

use core::pbrt::{Float, Options};
use core::parser;

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

macro_rules! verify_initialized {
    ($pbrt:expr, $func:expr) => (
        if $pbrt.current_api_state == APIState::Uninitialized {
            error!("Pbrt.init() must be before calling \"{}()\".  Ignoring.", $func);
            return;
        }
    )
}

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
pub struct Pbrt {
    opt: Options,
    current_api_state: APIState,
    // TODO(wathiede):
    // static TransformSet curTransform;
    // static uint32_t activeTransformBits = AllTransformsBits;
    // static std::map<std::string, TransformSet> namedCoordinateSystems;
    // static std::unique_ptr<RenderOptions> renderOptions;
    // static GraphicsState graphicsState;
    // static std::vector<GraphicsState> pushedGraphicsStates;
    // static std::vector<TransformSet> pushedTransforms;
    // static std::vector<uint32_t> pushedActiveTransformBits;
    // static TransformCache transformCache;
}

impl Pbrt {
    pub fn new(opt: Options) -> Pbrt {
        Pbrt {
            opt,
            current_api_state: APIState::Uninitialized,
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
            error!("Pbrt.init() has already been called.");
        }
        self.current_api_state = APIState::OptionsBlock;
    }

    pub fn cleaup(&mut self) {
        if self.current_api_state == APIState::Uninitialized {
            error!("Pbrt.cleanup() called without Pbrt.init().");
        } else if self.current_api_state == APIState::WorldBlock {
            error!("Pbrt.cleanup() called while inside world block.");
        }
        self.current_api_state = APIState::Uninitialized;
    }

    pub fn look_at(
        &mut self,
        ex: Float,
        ey: Float,
        ez: Float,
        lx: Float,
        ly: Float,
        lz: Float,
        ux: Float,
        uy: Float,
        uz: Float,
    ) {
        verify_initialized!(self, "Pbrt.look_at");
        info!(
            "eye: {:?} {:?} {:?} look: {:?} {:?} {:?} up: {:?} {:?} {:?}",
            ex, ey, ez, lx, ly, lz, ux, uy, uz
        );
    }
}
