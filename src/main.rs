use std::{env, fs};

struct Scanner {
    source: String,
    has_error: bool,
}

#[derive(Debug)]
struct Token {}

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

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn run_file(filename: String) {
    let contents = fs::read_to_string(filename).unwrap();
    run(contents);
}

fn main() {
    if env::args().len() != 2 {
        println!("Usage: lox [script]");
        std::process::exit(64);
    } else if env::args().len() == 2 {
        let args: Vec<_> = env::args().collect();
        run_file(args[1].clone());
    }
    //     run_
    // }
}
