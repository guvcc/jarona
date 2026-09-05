use std::env;
use std::fs;

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn run_file(path: &str) -> Result<(), String> {
    // 1. Read the file
    let source = fs::read_to_string(path)
        .map_err(|e| format!("could not read `{path}`: {e}"))?;

    // 2. Lex
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;

    // 3. Parse
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;

    // 4. Run
    let mut interpreter = Interpreter::new();
    interpreter.run(&program)?;

    Ok(())
}