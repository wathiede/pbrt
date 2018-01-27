use std::process;

#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::{Config, LogLevelFilter, TermLogger};
extern crate structopt;
use structopt::StructOpt;

extern crate pbrt;
use pbrt::core::api;

fn main() {
    let ref opt = pbrt::core::pbrt::Options::from_args();
    if opt.verbose {
        let _ = TermLogger::init(LogLevelFilter::Debug, Config::default());
    } else if opt.quiet {
        let _ = TermLogger::init(LogLevelFilter::Warn, Config::default());
    } else {
        let _ = TermLogger::init(LogLevelFilter::Info, Config::default());
    }
    if opt.scene_files.is_empty() {
        error!("One or more scene files required.");
        process::exit(1);
    }

    info!("Options: {:#?}", &opt);
    let ref pbrt = api::Pbrt::new(&opt);
    for f in &opt.scene_files {
        match pbrt.parse_file(&f) {
            Ok(res) => {
                if opt.verbose {
                    println!("Rendered {}\n{:#?}", f, res);
                }
            }
            Err(err) => {
                error!("Failed to parse {}: {:?}", f, err);
                process::exit(1);
            }
        }
    }
}
