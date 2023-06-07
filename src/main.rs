use interpreter::Interpreter;
use literal::Literal;
use parser::Parser;

use crate::scanner::Scanner;
use std::{env, fs, io::Write};

mod environment;
mod errors;
mod expr;
mod interpreter;
mod literal;
mod parser;
mod scanner;
mod stmt;
mod token;

fn run(interpreter: &mut Interpreter, source: String) -> Option<Literal> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    match tokens {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            let statements = parser.parse();
            let mut last: Option<Literal> = None;
            match statements {
                Ok(statements) => {
                    for stmt in statements {
                        match interpreter.execute(&stmt) {
                            Err(reason) => {
                                println!("{:?}", reason);
                                break;
                            }
                            Ok(result) => {
                                last = Some(result);
                            }
                        }
                    }
                    return last;
                }
                Err(reason) => {
                    println!("{:?}", reason);
                    return None;
                }
            }
        }
        Err(errors) => {
            for error in errors {
                println!("{:?}", error);
            }
            return None;
        }
    }
}

fn run_file(filename: String) {
    let contents = fs::read_to_string(filename).unwrap();
    let mut interpreter = Interpreter::new();
    run(&mut interpreter, contents);
}

fn run_prompt() {
    let mut buffer = String::new();
    let mut interpreter = Interpreter::new();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        match std::io::stdin().read_line(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                match run(&mut interpreter, buffer.clone()) {
                    Some(value) => {
                        println!("=> {}", value);
                    }
                    _ => (),
                }
                buffer.clear();
            }
            _ => {
                break;
            }
        }
    }
}

fn main() {
    if env::args().len() > 2 {
        println!("Usage: lox [script]");
        std::process::exit(64);
    } else if env::args().len() == 2 {
        let args: Vec<_> = env::args().collect();
        run_file(args[1].clone());
    } else {
        run_prompt();
    }
}
