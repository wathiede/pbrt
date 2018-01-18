use std::fs::File;
use std::io::Read;
use std::io;
use std::path::Path;

extern crate nom;
use self::nom::IResult;

use core::pbrt::Float;
use core::parser;

// Pbrt is the top-level global container for all rendering functionality.
#[derive(Debug)]
pub struct Pbrt {}

impl Pbrt {
    pub fn new() -> Pbrt {
        Pbrt {}
    }

    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<parser::Scene, io::Error> {
        let mut f = File::open(path)?;
        let mut buffer = Vec::new();

        // read the whole file
        f.read_to_end(&mut buffer)?;
        match parser::parse_scene(&buffer[..]) {
            IResult::Done(_, res) => Ok(res),
            IResult::Error(ref e) => panic!("e: {:#?}", e),
            IResult::Incomplete(n) => panic!("need: {:?}", n),
        }
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
