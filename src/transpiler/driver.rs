use std::io;
use std::io::Write;

use crate::transpiler::lexer::*;
use crate::transpiler::parser::*;

pub use self::Mode::{
    Interpreter,
    Transpiler
};

#[derive(PartialEq, Clone, Debug)]
pub enum Mode {
    Interpreter,
    Transpiler
}

#[derive(PartialEq, Clone, Debug)]
pub struct DriverConfig {
    pub display_tokens: bool,
    pub display_ast: bool
}

pub fn run(mode: Mode, display_settings: DriverConfig) {
    match mode {
        Mode::Interpreter => run_interpreter(display_settings),
        Mode::Transpiler => run_transpiler(display_settings)
    }
}

fn run_interpreter(display_settings: DriverConfig) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut parser_settings = default_parser_settings();

    'main: loop {
        print!("> ");
        stdout.flush().unwrap();
        input.clear();
        stdin.read_line(&mut input).ok().expect("Failed to read line");
        if input.as_str() == ".quit\n" {
            break;
        }

        let mut ast = Vec::new();
        let mut prev_tokens = Vec::new();
        
        loop {
            let tokens = tokenize(input.as_str());
            if display_settings.display_tokens {
                println!("Tokens: {:?}", tokens);
            }
            prev_tokens.extend(tokens.into_iter());

            let parsing_result = parse(prev_tokens.as_slice(), ast.as_slice(), &mut parser_settings);
            match parsing_result {
                Ok((parsed_ast, rest)) => {
                    ast.extend(parsed_ast.into_iter());
                    if rest.is_empty() {
                        break
                    } else {
                        prev_tokens = rest;
                    }
                },
                Err(message) => {
                    println!("Error occured: {}", message);
                    continue 'main
                }
            }
            print!(". ");
            stdout.flush().unwrap();
            input.clear();
            stdin.read_line(&mut input).ok().expect("Failed to read line");
        }

        if display_settings.display_ast {
            println!("AST: {:?}", ast);
            continue
        }
    }
}

fn run_transpiler(display_settings: DriverConfig) {

}
