use crate::token::Token;
use std::{env, fs};

mod token;

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Self { had_error: false }
    }

    fn report(mut self, line: isize, location: String, message: String) {
        println!("[line {line}] Error {location}: {message}");
        self.had_error = true;
    }

    fn run(mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    fn run_file(self, filename: String) {
        let contents = fs::read_to_string(filename).unwrap();
        self.run(contents);
    }
}

struct Scanner {
    source: String,
    has_error: bool,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            source,
            has_error: false,
        }
    }

    fn scan_tokens(self) -> Vec<Token> {
        unimplemented!()
    }
}

fn main() {
    if env::args().len() != 2 {
        println!("Usage: lox [script]");
        std::process::exit(64);
    } else if env::args().len() == 2 {
        let args: Vec<_> = env::args().collect();
        let mut lox = Lox::new();
        lox.run_file(args[1].clone());
    }
    //     run_
    // }
}
