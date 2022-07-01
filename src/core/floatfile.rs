// Copyright 2022 Google LLC
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
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

use crate::Float;

use thiserror::Error;

/// Error type for reading images from disk.
#[derive(Debug, Error)]
pub enum Error {
    /// Standard `io::Error` generated.
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    /// Standard `std::num::ParseFloatError`.
    #[error("float error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}

/// Read whitespace separated floats from file. Everything after a `#` on the line is ignored.
pub fn read_float_file(name: &str) -> Result<Vec<Float>, Error> {
    let buf = BufReader::new(File::open(name)?);
    let mut floats: Vec<Float> = Vec::new();
    for line in buf.lines() {
        let line = line?;
        // Strip comments from line before tokenizing.
        let line = if let Some(idx) = line.find('#') {
            &line[..idx]
        } else {
            &line[..]
        };
        eprintln!("line '{}'", line);
        for word in line.split_ascii_whitespace() {
            eprintln!("word '{}'", word);
            let f = word.parse()?;
            floats.push(f);
        }
    }
    Ok(floats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good() -> Result<(), Error> {
        let floats = read_float_file("src/core/testdata/good.floats")?;
        assert_eq!(floats, vec![1., 2.0, 0.5, 10.0, Float::INFINITY]);
        Ok(())
    }

    #[test]
    fn bad1() -> Result<(), Error> {
        assert!(read_float_file("src/core/testdata/bad1.floats").is_err());
        Ok(())
    }
}
