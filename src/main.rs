use crate::scanner::Scanner;
use std::{env, fs};

mod expr;
mod scanner;
mod token;

struct Lox;

impl Lox {
    fn new() -> Self {
        Self
    }

    fn run(self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        match tokens {
            Ok(result) => {
                for token in result.tokens {
                    println!("{:?}", token);
                }
            }
            Err(result) => {
                for error in result.errors {
                    println!("{:?}", error);
                }
            }
        }
    }

    fn run_file(self, filename: String) {
        let contents = fs::read_to_string(filename).unwrap();
        self.run(contents);
    }
}

fn main() {
    if env::args().len() != 2 {
        println!("Usage: lox [script]");
        std::process::exit(64);
    } else if env::args().len() == 2 {
        let args: Vec<_> = env::args().collect();
        let lox = Lox::new();
        lox.run_file(args[1].clone());
    }
    //     run_
    // }
}
