use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use super::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use super::lexer::Lexer;
use super::parser::Parser;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Null,
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: &Program) -> Result<(), String> {
        for statement in &program.statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), String> {
        match statement {
            Stmt::Var { name, value } => {
                let value = self.evaluate(value)?;

                self.variables.insert(name.clone(), value);

                Ok(())
            }

            Stmt::Assign { name, value } => {
                let value = self.evaluate(value)?;

                if !self.variables.contains_key(name) {
                    return Err(format!("undefined variable `{name}`"));
                }

                self.variables.insert(name.clone(), value);

                Ok(())
            }

            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;

                match value {
                    Value::Number(value) => {
                        println!("{value}");
                    }

                    Value::String(value) => {
                        println!("{value}");
                    }

                    Value::Array(values) => {
                        for value in values {
                            match value {
                                Value::String(value) => {
                                    println!("{value}");
                                }

                                Value::Number(value) => {
                                    println!("{value}");
                                }

                                Value::Array(_) => {
                                    println!("[array]");
                                }

                                Value::Null => {
                                    println!("null");
                                }
                            }
                        }
                    }

                    Value::Null => {
                        println!("null");
                    }
                }

                Ok(())
            }

            Stmt::Expr(expr) => {
                self.evaluate(expr)?;

                Ok(())
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(value) => {
                Ok(Value::Number(*value))
            }

            Expr::String(value) => {
                Ok(Value::String(value.clone()))
            }

            Expr::Import(path) => {
                let path = self.evaluate(path)?;

                let Value::String(path) = path else {
                    return Err(
                        "import path must be a string".to_string()
                    );
                };

                self.import_file(&path)?;

                Ok(Value::Null)
            }

            Expr::ImportStr(path) => {
                let path = self.evaluate(path)?;

                let Value::String(path) = path else {
                    return Err(
                        "import path must be a string".to_string()
                    );
                };

                let lines = self.read_file(&path)?;

                let values = lines
                    .into_iter()
                    .map(Value::String)
                    .collect();

                Ok(Value::Array(values))
            }

            Expr::Array(elements) => {
                let values = elements
                    .iter()
                    .map(|element| self.evaluate(element))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Value::Array(values))
            }

            Expr::Variable(name) => {
                match self.variables.get(name) {
                    Some(value) => Ok(value.clone()),

                    None => {
                        Err(format!("undefined variable `{name}`"))
                    }
                }
            }

            Expr::Unary { op, expr } => {
                let value = self.evaluate(expr)?;

                let Value::Number(value) = value else {
                    return Err(
                        "unary operator requires a number".to_string()
                    );
                };

                match op {
                    UnaryOp::Negate => {
                        Ok(Value::Number(-value))
                    }

                    UnaryOp::Plus => {
                        Ok(Value::Number(value))
                    }
                }
            }

            Expr::Binary { left, op, right } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                let Value::Number(left) = left else {
                    return Err(
                        "left side of binary expression must be a number"
                            .to_string()
                    );
                };

                let Value::Number(right) = right else {
                    return Err(
                        "right side of binary expression must be a number"
                            .to_string()
                    );
                };

                match op {
                    BinaryOp::Add => {
                        Ok(Value::Number(left + right))
                    }

                    BinaryOp::Subtract => {
                        Ok(Value::Number(left - right))
                    }

                    BinaryOp::Multiply => {
                        Ok(Value::Number(left * right))
                    }

                    BinaryOp::Divide => {
                        if right == 0.0 {
                            return Err(
                                "division by zero".to_string()
                            );
                        }

                        Ok(Value::Number(left / right))
                    }
                }
            }

            _ => {
                Err("expression not implemented yet".to_string())
            }
        }
    }

    fn import_file(&mut self, path: &str) -> Result<(), String> {
        let source = fs::read_to_string(path)
            .map_err(|e| format!("could not read `{path}`: {e}"))?;

        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let program = parser.parse_program()?;

        self.run(&program)?;

        Ok(())
    }

    fn read_file(&mut self, path: &str) -> Result<Vec<String>, String> {
        let file = File::open(path)
            .map_err(|e| format!("could not open `{path}`: {e}"))?;

        let reader = BufReader::new(file);
        let mut lines = Vec::new();

        for line in reader.lines() {
            let line = line
                .map_err(|e| format!("could not read line: {e}"))?;

            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            lines.push(line);
        }

        Ok(lines)
    }
}