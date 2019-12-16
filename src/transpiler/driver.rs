use std::io;
use std::io::{ Write, BufReader, BufRead, Error };
use std::fs::File;
use std::process::Command;

use crate::transpiler::lexer::*;
use crate::transpiler::parser::*;
use crate::transpiler::translator::*;

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
    pub display_ast: bool,
    pub build_and_run: bool
}

pub fn run(mode: Mode, config: DriverConfig) -> Result<(), Error> {
    match mode {
        Mode::Interpreter => run_interpreter(config),
        Mode::Transpiler => run_transpiler(config)
    }
}

fn run_interpreter(config: DriverConfig) -> Result<(), Error> {
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
            if config.display_tokens {
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

        if config.display_ast {
            println!("AST: {:?}", ast);
            continue
        }
    }

    Ok(())
}

fn run_transpiler(config: DriverConfig) -> Result<(), Error> {
    let source_file_path: &'static str = "example/input.txt";
    let mut output_file = File::create("example/output.rs")?;

    let input = File::open(source_file_path)?;
    let buffered = BufReader::new(input);

    let mut ast = Vec::new();
    let mut parser_settings = default_parser_settings();
    let mut source_code = String::new();

    for line in buffered.lines() {
        let code_line = line?;
        source_code.push_str(code_line.as_str());
        source_code.push_str(" ");
    }

    let tokens = tokenize(source_code.as_str());
    if config.display_tokens {
        println!("Tokens: {:?}", tokens);
    }

    let parsing_result = parse(tokens.as_slice(), ast.as_slice(), &mut parser_settings);
    match parsing_result {
        Ok((parsed_ast, _)) => {
            ast = parsed_ast;
        },
        Err(message) => {
            println!("Error occured: {}", message);
        }
    }

    if config.display_ast {
        println!("AST: {:?}", ast);
    }

    // Translating from mini language to Rust source code
    let rust_source_code: String = match translate(&ast) {
        Ok(source_code) => source_code,
        Err(message) => panic!("{:?}", message)
    };

    // Storing transpiled Rust source code to output file
    output_file.write_all(rust_source_code.as_bytes())?;

    // Building and running transpiled Rust source code
    if config.build_and_run {
        Command::new("rustc")
                .arg("example/output.rs")
                .spawn()
                .expect("Command failed.");
    }

    Ok(())
}
