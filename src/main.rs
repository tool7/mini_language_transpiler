extern crate docopt;
extern crate serde;

mod transpiler;

use serde::Deserialize;
use docopt::Docopt;

use transpiler::driver;
use transpiler::driver::{ Mode, DriverConfig };

const USAGE: &'static str = "
Usage: mini_language_transpiler [(-i | -t)]

Options:
    -i  Run program in interpreter mode.
    -t  Run program in transpiler mode.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_i: bool,
    flag_t: bool
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mode = 
        if args.flag_i { Mode::Interpreter }
        else if args.flag_t { Mode::Transpiler }
        else { Mode::Transpiler };

    let config = DriverConfig {
        display_tokens: false,
        display_ast: false,
        build_and_run: false
    };

    driver::run(mode, config);
}
