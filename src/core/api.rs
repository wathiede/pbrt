use std::fs::File;
use std::io::Read;
use std::io;
use std::path::Path;

extern crate nom;

use core::pbrt::Float;
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

// Pbrt is the top-level global container for all rendering functionality.
#[derive(Debug)]
pub struct Pbrt {}

impl Pbrt {
    pub fn new() -> Pbrt {
        Pbrt {}
    }

    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<parser::Scene, Error> {
        let mut f = File::open(path)?;
        let mut buffer = Vec::new();

        // read the whole file
        f.read_to_end(&mut buffer)?;
        let scene = parser::parse_scene(&buffer[..])?;
        Ok(scene)
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
    ) -> &mut Self {
        println!(
            "eye: {:?} {:?} {:?} look: {:?} {:?} {:?} up: {:?} {:?} {:?}",
            ex, ey, ez, lx, ly, lz, ux, uy, uz
        );
        self
    }
}
