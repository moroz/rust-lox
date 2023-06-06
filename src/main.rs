use interpreter::EvaluationResult;
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
mod token;

struct Lox;

impl Lox {
    fn new() -> Self {
        Self
    }

    fn run(&self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        match tokens {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                let statements = parser.parse();
                for stmt in statements {
                    match stmt.evaluate() {
                        Err(reason) => {
                            println!("{:?}", reason);
                            break;
                        }
                        _ => (),
                    }
                }
            }
            Err(errors) => {
                for error in errors {
                    println!("{:?}", error);
                }
            }
        }
    }

    fn run_file(self, filename: String) {
        let contents = fs::read_to_string(filename).unwrap();
        self.run(contents);
    }

    fn run_prompt(&self) {
        let mut buffer = String::new();

        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            match std::io::stdin().read_line(&mut buffer) {
                Ok(0) => {
                    break;
                }
                Ok(_) => {
                    self.run(buffer.clone());
                    buffer.clear();
                }
                _ => {
                    break;
                }
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
        let lox = Lox::new();
        lox.run_file(args[1].clone());
    } else {
        let lox = Lox::new();
        lox.run_prompt();
    }
}
