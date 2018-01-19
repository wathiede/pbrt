extern crate getopts;
use getopts::Options;
use std::env;

extern crate pbrt;
use pbrt::core::api;

fn print_usage(program: &str, opts: Options) {
    println!(
        "{}",
        opts.usage(&format!(
            "Usage: {} [options] <scene file 1> [.. scene file N]",
            program
        ))
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this usage message.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.is_empty() {
        print_usage(program, opts);
    }

    let ref pbrt = api::Pbrt::new();
    for f in matches.free {
        match pbrt.parse_file(&f) {
            Ok(res) => println!("Rendered {}\n{:#?}", f, res),
            Err(err) => panic!("Failed to parse {}: {:?}", f, err),
        }
    }
}
