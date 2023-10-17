use std::{io::{self, Write, Read}, env::args, fs::File};

use crate::{interpreter::Interpreter, parse::Parser, token::Lexer};

mod interpreter;
mod parse;
mod token;
mod types;

fn main() {
    let mut interpreter = Interpreter::new();
    let file_path = args().nth(1);
    if let Some(file_path) = file_path {
        run_file(&mut interpreter, &file_path);
    } else {
        repl(&mut interpreter);
    }
}

fn run(interpreter: &mut Interpreter, code: &str) {
    let lexer = Lexer::new(code.chars().collect());
    let mut parser = Parser::new(lexer);
    let program = parser.parse();

    // println!("{:?}", program);

    if let Some(program) = program {
        interpreter.run(&program);
    }
}

fn run_file(interpreter: &mut Interpreter, file_path: &str) {
    let mut file = File::open(file_path).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    run(interpreter, &code);
}

/// 対話型
fn repl(interpreter: &mut Interpreter) {
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

        run(interpreter, &code);
    }
}