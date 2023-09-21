use std::{fs, env};

mod tokenizer;
mod variables;

mod interpreter;
use interpreter::interpret;

mod debugger;
#[allow(unused)]
use debugger::debug_interpret;


use macro_clap::*;

cli!(
    const ARG_PARSER: ArgParser<""> = [
        arg!(file_name as String),
        opt!(options as RunningOptions {
            cwd: ["--cwd", "--dir"] -> (GrabFirst<String>)
        })
    ]
);

fn main() {
    let (file_path, options) = match ARG_PARSER.parse_args() {
        Ok(params) => params,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    match options.cwd.first {
        Some(path) => match env::set_current_dir(path) {
            Ok(()) => (),
            Err(e) => {
                println!("Could not change directory: {e}");
                return;
            }
        },
        None => ()
    };
    match fs::read_to_string(file_path) {
        Ok(contents) => interpret(contents),
        Err(e) => {
            println!("Could not run program: {e}");
            return;
        }
    };
}