use std::fs::File;
use std::io::Read;
use std::io;
use std::path::Path;

use core::pbrt::Float;

// Pbrt is the top-level global container for all rendering functionality.
#[derive(Debug)]
pub struct Pbrt {}

impl Pbrt {
    pub fn new() -> Pbrt {
        Pbrt {}
    }

    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<String, io::Error> {
        let mut f = File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s).map(|_| s)
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
        self
    }
}
