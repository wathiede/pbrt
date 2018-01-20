#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::{Config, LogLevelFilter, TermLogger};

extern crate getopts;
use std::env;

extern crate pbrt;
use pbrt::core::api;

fn print_usage(program: &str, opts: getopts::Options) {
    println!(
        "{}",
        opts.usage(&format!(
            r#"Usage: {} [options] <scene file 1> [.. scene file N]
[NI] Not Implemented and will likely never be implemented.
"#,
            program
        ))
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = getopts::Options::new();
    opts.optflag("","cat","[NI] Print a reformatted version of the input file(s) to standard output. Does not render an image.");
    opts.optflag("", "help", "Print this help text.");
    opts.optopt(
        "n",
        "nthreads",
        "Use specified number of threads for rendering.",
        "num",
    );
    opts.optopt(
        "o",
        "outfile",
        "Write the final image to the given filename.",
        "filename",
    );
    opts.optflag(
        "",
        "quick",
        "Automatically reduce a number of quality settings to render more quickly.",
    );
    opts.optflag(
        "q",
        "quiet",
        "Suppress all text output other than error messages.",
    );
    opts.optflag("","toply","[NI] Print a reformatted version of the input file(s) to standard output and convert all triangle meshes to PLY files. Does not render an image.");
    opts.optflag(
        "v",
        "verbose",
        "Print out more detailed logging information.",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if matches.opt_present("help") {
        print_usage(&program, opts);
        return;
    }

    let opt = pbrt::core::pbrt::Options {
        num_threads: matches
            .opt_str("nthreads")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default(),
        quick_render: matches.opt_present("quick"),
        quiet: matches.opt_present("quiet"),
        verbose: matches.opt_present("verbose"),
        image_file: matches.opt_str("outfile").unwrap_or_default(),
    };
    if opt.verbose {
        let _ = TermLogger::init(LogLevelFilter::Debug, Config::default());
    } else if opt.quiet {
        let _ = TermLogger::init(LogLevelFilter::Warn, Config::default());
    } else {
        let _ = TermLogger::init(LogLevelFilter::Info, Config::default());
    }

    if matches.free.is_empty() {
        print_usage(program, opts);
        return;
    }

    info!("Options: {:#?}", &opt);
    info!("Inputs: {:?}", &matches.free);
    let ref pbrt = api::Pbrt::new(opt.clone());
    for f in matches.free {
        match pbrt.parse_file(&f) {
            Ok(res) => {
                if opt.verbose {
                    println!("Rendered {}\n{:#?}", f, res);
                }
            }
            Err(err) => panic!("Failed to parse {}: {:?}", f, err),
        }
    }
}
