use std::fs;

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

pub fn run_file(path: &str) -> Result<(), String> {
    let source = fs::read_to_string(path)
        .map_err(|e| format!("could not read `{path}`: {e}"))?;

    // 2. Lex
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;


    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;


    let mut interpreter = Interpreter::new();
    interpreter.run(&program)?;

    Ok(())
}