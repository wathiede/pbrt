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

use log::error;
use log::info;
use simplelog::{Config, LogLevelFilter, TermLogger};

use structopt;
use structopt::StructOpt;

use pbrt;
use pbrt::core::api;

#[derive(Clone, Debug, Default, StructOpt)]
#[structopt(name = "pbrt", about = "Rust implementation of http://pbrt.org/")]
pub struct Options {
    #[structopt(short = "n", long = "nthreads")]
    /// Use specified number of threads for rendering.
    pub num_threads: Option<u32>,
    #[structopt(long = "quick")]
    /// Automatically reduce a number of quality settings to render more quickly.
    pub quick_render: bool,
    #[structopt(short = "q", long = "quiet")]
    /// Suppress all text output other than error messages.
    pub quiet: bool,
    #[structopt(short = "v", long = "verbose")]
    /// Print out more detailed logging information.
    pub verbose: bool,
    #[structopt(short = "o", long = "outfile")]
    /// Write the final image to the given filename.
    pub image_file: Option<String>,
    pub scene_files: Vec<String>,
}

fn main() {
    let flags = Options::from_args();
    if flags.verbose {
        let _ = TermLogger::init(LogLevelFilter::Debug, Config::default());
    } else if flags.quiet {
        let _ = TermLogger::init(LogLevelFilter::Warn, Config::default());
    } else {
        let _ = TermLogger::init(LogLevelFilter::Info, Config::default());
    }
    if flags.scene_files.is_empty() {
        println!("One or more scene files required.\n");
        Options::clap().print_help().unwrap();
        process::exit(1);
    }

    info!("Options: {:#?}", &flags);
    let opts = pbrt::Options {
        num_threads: flags.num_threads.unwrap_or(1),
        quick_render: flags.quick_render,
        quiet: flags.quiet,
        verbose: flags.verbose,
        image_file: flags.image_file.unwrap_or("".to_owned()),
    };
    let ref mut pbrt = api::Pbrt::new(opts.clone());
    pbrt.init();
    for f in &flags.scene_files {
        match pbrt.parse_file(&f) {
            Ok(_) => {
                if opts.verbose {
                    println!("Rendered {}\n{:#?}", f, pbrt);
                }
            }
            Err(err) => {
                error!("Failed to parse {}: {:?}", f, err);
                process::exit(1);
            }
        }
    }
    pbrt.cleaup();
}
