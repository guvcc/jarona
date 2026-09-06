use std::collections::HashMap;
use std::fs;

use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use crate::lexer::Lexer;
use crate::parser::Parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Null,

    Struct {
        name: String,
        fields: HashMap<String, Value>,
    },
}

#[derive(Debug, Clone)]
struct Function {
    params: Vec<String>,
    body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
struct Struct {
    fields: Vec<String>,
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    structs: HashMap<String, Struct>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: &Program) -> Result<(), String> {
        for statement in &program.statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<Option<Value>, String> {
        match statement {
            Stmt::Var { name, value } => {
                let value = self.evaluate(value)?;

                self.variables.insert(
                    name.clone(),
                    value,
                );

                Ok(None)
            }

            Stmt::Assign { name, value } => {
                let value = self.evaluate(value)?;

                self.variables.insert(
                    name.clone(),
                    value,
                );

                Ok(None)
            }

            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;

                self.print_value(&value);

                Ok(None)
            }

            Stmt::Expr(expr) => {
                self.evaluate(expr)?;

                Ok(None)
            }

            Stmt::Struct {
                name,
                fields,
            } => {
                let structure = Struct {
                    fields: fields.clone(),
                };

                self.structs.insert(
                    name.clone(),
                    structure,
                );

                Ok(None)
            }

            Stmt::Void {
                name,
                params,
                body,
            } => {
                let function = Function {
                    params: params.clone(),
                    body: body.clone(),
                };

                self.functions.insert(
                    name.clone(),
                    function,
                );

                Ok(None)
            }

            Stmt::Return(expr) => {
                let value = self.evaluate(expr)?;

                Ok(Some(value))
            }

            Stmt::If {
                condition,
                body,
                else_body,
            } => {
                let condition = self.evaluate(condition)?;

                match condition {
                    Value::Boolean(true) => {
                        for statement in body {
                            if let Some(value) = self.execute(statement)? {
                                return Ok(Some(value));
                            }
                        }
                    }

                    Value::Boolean(false) => {
                        if let Some(else_body) = else_body {
                            for statement in else_body {
                                if let Some(value) = self.execute(statement)? {
                                    return Ok(Some(value));
                                }
                            }
                        }
                    }

                    _ => {
                        return Err(
                            "if condition must be a boolean".to_string()
                        );
                    }
                }

                Ok(None)
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

            Expr::Boolean(value) => {
                Ok(Value::Boolean(*value))
            }

            Expr::Variable(name) => {
                self.variables
                    .get(name)
                    .cloned()
                    .ok_or_else(|| {
                        format!("undefined variable `{name}`")
                    })
            }

            Expr::Array(elements) => {
                let mut values = Vec::new();

                for element in elements {
                    values.push(
                        self.evaluate(element)?
                    );
                }

                Ok(Value::Array(values))
            }

            Expr::Binary {
                left,
                op,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                self.evaluate_binary(
                    left,
                    *op,
                    right,
                )
            }

            Expr::Unary {
                op,
                expr,
            } => {
                let value = self.evaluate(expr)?;

                self.evaluate_unary(
                    *op,
                    value,
                )
            }

            Expr::Call {
                name,
                args,
            } => {
                if self.structs.contains_key(name) {
                    self.construct_struct(
                        name,
                        args,
                    )
                } else {
                    self.call_function(
                        name,
                        args,
                    )
                }
            }

            Expr::Field {
                object,
                name,
            } => {
                let object = self.evaluate(object)?;

                match object {
                    Value::Struct {
                        name: struct_name,
                        fields,
                    } => {
                        fields
                            .get(name)
                            .cloned()
                            .ok_or_else(|| {
                                format!(
                                    "struct `{struct_name}` has no field `{name}`"
                                )
                            })
                    }

                    _ => {
                        Err(
                            format!(
                                "cannot access field `{name}` on non-struct value"
                            )
                        )
                    }
                }
            }

            Expr::Import(expr) => {
                let path = self.evaluate(expr)?;

                match path {
                    Value::String(path) => {
                        self.import_file(&path)
                    }

                    _ => {
                        Err(
                            "import path must be a string".to_string()
                        )
                    }
                }
            }

            Expr::ImportStr(expr) => {
                let path = self.evaluate(expr)?;

                match path {
                    Value::String(path) => {
                        let contents = fs::read_to_string(&path)
                            .map_err(|e| {
                                format!(
                                    "could not read `{path}`: {e}"
                                )
                            })?;

                        Ok(Value::String(contents))
                    }

                    _ => {
                        Err(
                            "import$str path must be a string".to_string()
                        )
                    }
                }
            }

            Expr::Declare(expr) => {
                let path = self.evaluate(expr)?;

                match path {
                    Value::String(path) => {
                        self.declare_file(&path)?;

                        Ok(Value::Null)
                    }

                    _ => {
                        Err(
                            "declare path must be a string".to_string()
                        )
                    }
                }
            }
        }
    }

    fn construct_struct(
        &mut self,
        name: &str,
        args: &[Expr],
    ) -> Result<Value, String> {
        let fields = self
            .structs
            .get(name)
            .ok_or_else(|| {
                format!("undefined struct `{name}`")
            })?
            .fields
            .clone();

        if args.len() != fields.len() {
            return Err(
                format!(
                    "struct `{name}` expected {} fields, got {}",
                    fields.len(),
                    args.len()
                )
            );
        }

        let mut values = HashMap::new();

        for (field, arg) in fields.iter().zip(args) {
            let value = self.evaluate(arg)?;

            values.insert(
                field.clone(),
                value,
            );
        }

        Ok(Value::Struct {
            name: name.to_string(),
            fields: values,
        })
    }

    fn call_function(
        &mut self,
        name: &str,
        args: &[Expr],
    ) -> Result<Value, String> {
        let function = self
            .functions
            .get(name)
            .ok_or_else(|| {
                format!("undefined function `{name}`")
            })?
            .clone();

        if args.len() != function.params.len() {
            return Err(
                format!(
                    "function `{name}` expected {} arguments, got {}",
                    function.params.len(),
                    args.len()
                )
            );
        }

        for (param, arg) in function.params.iter().zip(args) {
            let value = self.evaluate(arg)?;

            self.variables.insert(
                param.clone(),
                value,
            );
        }

        for statement in &function.body {
            if let Some(value) = self.execute(statement)? {
                return Ok(value);
            }
        }

        Ok(Value::Null)
    }

    fn evaluate_binary(
        &self,
        left: Value,
        op: BinaryOp,
        right: Value,
    ) -> Result<Value, String> {
        match (left, op, right) {
            (
                Value::Number(a),
                BinaryOp::Add,
                Value::Number(b),
            ) => Ok(Value::Number(a + b)),

            (
                Value::Number(a),
                BinaryOp::Subtract,
                Value::Number(b),
            ) => Ok(Value::Number(a - b)),

            (
                Value::Number(a),
                BinaryOp::Multiply,
                Value::Number(b),
            ) => Ok(Value::Number(a * b)),

            (
                Value::Number(a),
                BinaryOp::Divide,
                Value::Number(b),
            ) => Ok(Value::Number(a / b)),

            (
                Value::Number(a),
                BinaryOp::Modulo,
                Value::Number(b),
            ) => Ok(Value::Number(a % b)),

            (a, BinaryOp::Equal, b) => {
                Ok(Value::Boolean(a == b))
            }

            _ => {
                Err(
                    "invalid binary operation".to_string()
                )
            }
        }
    }

    fn evaluate_unary(
        &self,
        op: UnaryOp,
        value: Value,
    ) -> Result<Value, String> {
        match (op, value) {
            (
                UnaryOp::Negate,
                Value::Number(value),
            ) => {
                Ok(Value::Number(-value))
            }

            (
                UnaryOp::Plus,
                Value::Number(value),
            ) => {
                Ok(Value::Number(value))
            }

            _ => {
                Err(
                    "invalid unary operation".to_string()
                )
            }
        }
    }

    fn print_value(&self, value: &Value) {
        match value {
            Value::Number(value) => {
                println!("{value}");
            }

            Value::String(value) => {
                println!("{value}");
            }

            Value::Boolean(value) => {
                println!("{value}");
            }

            Value::Array(values) => {
                println!("{values:?}");
            }

            Value::Null => {
                println!("null");
            }

            Value::Struct {
                name,
                fields,
            } => {
                println!("{name} {{");

                for (field, value) in fields {
                    print!("    {field}: ");
                    self.print_value(value);
                }

                println!("}}");
            }
        }
    }

    fn import_file(
        &mut self,
        path: &str,
    ) -> Result<Value, String> {
        let source = fs::read_to_string(path)
            .map_err(|e| {
                format!(
                    "could not read `{path}`: {e}"
                )
            })?;

        let mut lexer = Lexer::new(&source);

        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);

        let program = parser.parse_program()?;

        for statement in &program.statements {
            self.execute(statement)?;
        }

        Ok(Value::Null)
    }

    fn declare_file(
        &mut self,
        path: &str,
    ) -> Result<(), String> {
        let source = fs::read_to_string(path)
            .map_err(|e| {
                format!(
                    "could not read `{path}`: {e}"
                )
            })?;

        let mut lexer = Lexer::new(&source);

        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);

        let program = parser.parse_program()?;

        for statement in program.statements {
            match statement {
                Stmt::Var {
                    name,
                    value,
                } => {
                    let value = self.evaluate(&value)?;

                    self.variables.insert(
                        name,
                        value,
                    );
                }

                Stmt::Void {
                    name,
                    params,
                    body,
                } => {
                    self.functions.insert(
                        name,
                        Function {
                            params,
                            body,
                        },
                    );
                }

                Stmt::Struct {
                    name,
                    fields,
                } => {
                    self.structs.insert(
                        name,
                        Struct {
                            fields,
                        },
                    );
                }

                _ => {
                    return Err(
                        format!(
                            "cannot declare this statement from `{path}`"
                        )
                    );
                }
            }
        }

        Ok(())
    }
}