use environment::Environment;
use interpreter::Interpreter;
use literal::Literal;
use parser::Parser;

use crate::scanner::Scanner;
use std::{cell::RefCell, env, fs, io::Write};

mod environment;
mod errors;
mod expr;
mod interpreter;
mod literal;
mod parser;
mod scanner;
mod token;

fn run(env: &RefCell<Environment>, source: String) -> Option<Literal> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut interpreter = Interpreter::new();

    match tokens {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            let statements = parser.parse();
            let mut last: Option<Literal> = None;
            match statements {
                Ok(statements) => {
                    for stmt in statements {
                        match interpreter.evaluate_statement(&env, stmt) {
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

fn run_file(env: &RefCell<Environment>, filename: String) {
    let contents = fs::read_to_string(filename).unwrap();
    run(env, contents);
}

fn run_prompt(env: &RefCell<Environment>) {
    let mut buffer = String::new();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        match std::io::stdin().read_line(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                match run(env, buffer.clone()) {
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
        let env = RefCell::new(Environment::new());
        run_file(&env, args[1].clone());
    } else {
        let env = RefCell::new(Environment::new());
        run_prompt(&env);
    }
}
