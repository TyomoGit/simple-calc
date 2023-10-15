use std::{io::{self, Write, Read}, env::args, fs::File};

use crate::{eval::Evaluator, parse::Parser, token::Lexer};

mod eval;
mod parse;
mod token;

fn main() {
    let mut evaluator = Evaluator::new();
    let file_path = args().nth(1);
    if let Some(file_path) = file_path {
        run(&mut evaluator, &file_path);
    } else {
        repl(&mut evaluator);
    }
}

fn run(evaluator: &mut Evaluator, file_path: &str) {
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    for line in content.lines() {
        let lexer = Lexer::new(line.chars().collect());
        let mut parser = Parser::new(lexer);
        let expr = parser.parse();
        if let Some(expr) = expr {
            println!("{}", evaluator.eval(&expr));
        }
    }
}

/// 対話型
fn repl(evaluator: &mut Evaluator) {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .expect("failed to read line");

        if code == "exit\n" {
            break;
        }

        let lexer = Lexer::new(code.chars().collect());

        let mut parser = Parser::new(lexer);

        let expr = parser.parse();
        
        // println!("\n{:?}", expr);

        if let Some(expr) = expr {
            println!("{}", evaluator.eval(&expr));
        }
    }
}