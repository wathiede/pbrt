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
use std::process;

use log::{error, info};

use clap::Parser;
use pbrt::{
    self,
    core::api::{PbrtAPI, API},
};

#[derive(Clone, Debug, Default, Parser)]
#[command(name = "pbrt", about = "Rust implementation of http://pbrt.org/")]
pub struct Options {
    #[arg(short = 'n', long = "nthreads")]
    /// Use specified number of threads for rendering.
    pub num_threads: Option<u32>,
    #[arg(long = "quick")]
    /// Automatically reduce a number of quality settings to render more quickly.
    pub quick_render: bool,
    #[arg(short = 'q', long = "quiet")]
    /// Suppress all text output other than error messages.
    pub quiet: bool,
    #[arg(short = 'v', long = "verbose")]
    /// Print out more detailed logging information.
    pub verbose: bool,
    #[arg(short = 'o', long = "outfile")]
    /// Write the final image to the given filename.
    pub image_file: Option<String>,
    #[arg(required = true)]
    pub scene_files: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = Options::parse();
    let verbosity = if flags.verbose {
        // Enable DEBUG logging.
        3
    } else if flags.quiet {
        // Only WARN and higher.
        1
    } else {
        // Default to INFO.
        2
    };

    stderrlog::new()
        .verbosity(verbosity)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()?;

    info!("Options: {:#?}", &flags);
    let opts = pbrt::Options {
        num_threads: flags.num_threads.unwrap_or(1),
        quick_render: flags.quick_render,
        quiet: flags.quiet,
        verbose: flags.verbose,
        image_file: flags.image_file.unwrap_or_else(|| "".to_owned()),
    };
    let pbrt = &mut PbrtAPI::from(opts.clone());
    pbrt.init();
    for f in &flags.scene_files {
        if let Err(err) = pbrt.parse_file(&f) {
            error!("Faild to parse '{f}': {err}");
            process::exit(1);
        }
        if opts.verbose {
            println!("Rendered {}\n{:#?}", f, pbrt);
        }
    }
    pbrt.cleanup();
    Ok(())
}
