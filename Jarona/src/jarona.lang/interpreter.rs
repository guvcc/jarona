use std::collections::HashMap;

use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};

pub struct Interpreter {
    variables: HashMap<String, f64>,
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

            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;

                println!("{value}");

                Ok(())
            }

            Stmt::Expr(expr) => {
                self.evaluate(expr)?;

                Ok(())
            }
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<f64, String> {
        match expr {
            Expr::Number(value) => Ok(*value),

            Expr::Variable(name) => {
                match self.variables.get(name) {
                    Some(value) => Ok(*value),

                    None => Err(format!("undefined variable `{name}`")),
                }
            }

            Expr::Unary { op, expr } => {
                let value = self.evaluate(expr)?;

                match op {
                    UnaryOp::Negate => Ok(-value),
                    UnaryOp::Plus => Ok(value),
                }
            }

            Expr::Binary { left, op, right } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match op {
                    BinaryOp::Add => Ok(left + right),
                    BinaryOp::Subtract => Ok(left - right),
                    BinaryOp::Multiply => Ok(left * right),
                    BinaryOp::Divide => {
                        if right == 0.0 {
                            return Err("division by zero".to_string());
                        }

                        Ok(left / right)
                    }
                }
            }

            _ => Err("expression not implemented yet".to_string()),
        }
    }
}