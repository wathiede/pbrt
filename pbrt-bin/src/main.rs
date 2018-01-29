use std::process;

#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::{Config, LogLevelFilter, TermLogger};

extern crate structopt;
#[macro_use]
extern crate structopt_derive;
use structopt::StructOpt;

extern crate pbrt;
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
        error!("One or more scene files required.");
        process::exit(1);
    }

    info!("Options: {:#?}", &flags);
    let opts = pbrt::core::pbrt::Options {
        num_threads: flags.num_threads.unwrap_or(1),
        quick_render: flags.quick_render,
        quiet: flags.quiet,
        verbose: flags.verbose,
        image_file: flags.image_file.unwrap_or("".to_owned()),
    };
    let ref mut pbrt = api::Pbrt::new(&opts);
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
