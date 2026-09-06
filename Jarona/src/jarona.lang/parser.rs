use crate::ast::{BinaryOp, Expr, Program, Stmt, UnaryOp};
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement()?);
        }

        Ok(Program { statements })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.matches(&TokenKind::Var) {
            return self.var_statement();
        }

        if self.matches(&TokenKind::If) {
            return self.if_statement();
        }

        if self.matches(&TokenKind::Void) {
            return self.void_statement();
        }

        if self.matches(&TokenKind::Struct) {
            return self.struct_statement();
        }

        if self.matches(&TokenKind::Return) {
            let value = self.expression()?;

            self.consume(
                &TokenKind::Semicolon,
                "expected `;` after return value",
            )?;

            return Ok(Stmt::Return(value));
        }

        if self.matches(&TokenKind::Print) {
            self.consume(
                &TokenKind::LParen,
                "expected `(` after `print`",
            )?;

            let value = self.expression()?;

            self.consume(
                &TokenKind::RParen,
                "expected `)` after print expression",
            )?;

            self.consume(
                &TokenKind::Semicolon,
                "expected `;` after print statement",
            )?;

            return Ok(Stmt::Print(value));
        }

        // x = expression;
        if let TokenKind::Ident(name) = self.peek().kind.clone() {
            if self
                .tokens
                .get(self.current + 1)
                .map(|token| &token.kind)
                == Some(&TokenKind::Equal)
            {
                self.advance();
                self.advance();

                let value = self.expression()?;

                self.consume(
                    &TokenKind::Semicolon,
                    "expected `;` after assignment",
                )?;

                return Ok(Stmt::Assign {
                    name,
                    value,
                });
            }
        }

        let expr = self.expression()?;

        self.consume(
            &TokenKind::Semicolon,
            "expected `;` after expression",
        )?;

        Ok(Stmt::Expr(expr))
    }

    fn struct_statement(&mut self) -> Result<Stmt, String> {
        let name = match self.advance().kind.clone() {
            TokenKind::Ident(name) => name,
            _ => {
                return Err(
                    self.error(
                        "expected identifier after `struct`",
                    )
                );
            }
        };

        self.consume(
            &TokenKind::LBrace,
            "expected `{` after if name",
        )?;

        let mut fields = Vec::new();

        while !self.check(&TokenKind::RBrace) {
            if self.is_at_end() {
                return Err(
                    self.error(
                        "expected `)` after parameters",
                    )
                );
            }

            match self.advance().kind.clone() {
                TokenKind::Ident(param) => {
                    fields.push(param);

                    self.consume(
                        &TokenKind::Semicolon,
                        "expected `;` after field",
                    )?;
                }

                _ => {
                    return Err(
                        self.error(
                            "expected parameter name",
                        )
                    );
                }
            }
        }

        self.consume(
            &TokenKind::RBrace,
            "expected `)` after parameters",
        )?;

        Ok(Stmt::Struct {
            name,
            fields,
        })
    }

    fn var_statement(&mut self) -> Result<Stmt, String> {
        let name = match self.advance().kind.clone() {
            TokenKind::Ident(name) => name,
            _ => {
                return Err(
                    self.error(
                        "expected identifier after `var`",
                    )
                );
            }
        };

        self.consume(
            &TokenKind::Equal,
            "expected `=` after variable name",
        )?;

        let value = self.expression()?;

        self.consume(
            &TokenKind::Semicolon,
            "expected `;` after variable declaration",
        )?;

        Ok(Stmt::Var {
            name,
            value,
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        let condition = self.expression()?;

        self.consume(
            &TokenKind::LBrace,
            "expected `{` after if condition",
        )?;

        let mut body = Vec::new();

        while !self.check(&TokenKind::RBrace) {
            if self.is_at_end() {
                return Err(
                    self.error(
                        "expected `}` after if body",
                    )
                );
            }

            body.push(self.statement()?);
        }

        self.consume(
            &TokenKind::RBrace,
            "expected `}` after if body",
        )?;

        let else_body = if self.matches(&TokenKind::Else) {
            self.consume(
                &TokenKind::LBrace,
                "expected `{` after `else`",
            )?;

            let mut else_body = Vec::new();

            while !self.check(&TokenKind::RBrace) {
                if self.is_at_end() {
                    return Err(
                        self.error(
                            "expected `}` after else body",
                        )
                    );
                }

                else_body.push(self.statement()?);
            }

            self.consume(
                &TokenKind::RBrace,
                "expected `}` after else body",
            )?;

            Some(else_body)
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            body,
            else_body,
        })
    }

    fn void_statement(&mut self) -> Result<Stmt, String> {
        let name = match self.advance().kind.clone() {
            TokenKind::Ident(name) => name,
            _ => {
                return Err(
                    self.error(
                        "expected a function name",
                    )
                );
            }
        };

        self.consume(
            &TokenKind::LParen,
            "expected `(` after void name",
        )?;

        let mut params = Vec::new();

        while !self.check(&TokenKind::RParen) {
            if self.is_at_end() {
                return Err(
                    self.error(
                        "expected `)` after parameters",
                    )
                );
            }

            match self.advance().kind.clone() {
                TokenKind::Ident(param) => {
                    params.push(param);
                }

                _ => {
                    return Err(
                        self.error(
                            "expected parameter name",
                        )
                    );
                }
            }
        }

        self.consume(
            &TokenKind::RParen,
            "expected `)` after parameters",
        )?;

        self.consume(
            &TokenKind::LBrace,
            "expected `{` after function declaration",
        )?;

        let mut body = Vec::new();

        while !self.check(&TokenKind::RBrace) {
            if self.is_at_end() {
                return Err(
                    self.error(
                        "expected `}` after void body",
                    )
                );
            }

            body.push(self.statement()?);
        }

        self.consume(
            &TokenKind::RBrace,
            "expected `}` after void body",
        )?;

        Ok(Stmt::Void {
            name,
            params,
            body,
        })
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.comparison()
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.matches(&TokenKind::EqualEqual) {
            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Equal,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.matches_any(&[
            TokenKind::Plus,
            TokenKind::Minus,
        ]) {
            let op = match self.previous().kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };

            let right = self.factor()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.matches(&TokenKind::Star)
            || self.matches(&TokenKind::Slash)
            || self.matches(&TokenKind::Percent)
        {
            let operator = match self.previous().kind {
                TokenKind::Star => BinaryOp::Multiply,
                TokenKind::Slash => BinaryOp::Divide,
                TokenKind::Percent => BinaryOp::Modulo,
                _ => unreachable!(),
            };

            let right = self.unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.matches_any(&[
            TokenKind::Minus,
            TokenKind::Plus,
        ]) {
            let op = match self.previous().kind {
                TokenKind::Minus => UnaryOp::Negate,
                TokenKind::Plus => UnaryOp::Plus,
                _ => unreachable!(),
            };

            let expr = self.unary()?;

            return Ok(Expr::Unary {
                op,
                expr: Box::new(expr),
            });
        }

        self.call()
    }

    // hello()
    // hello(10)
    // hello(x 10)
    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(&TokenKind::LParen) {
                let name = match expr {
                    Expr::Variable(name) => name,
                    _ => {
                        return Err(
                            self.error(
                                "only functions can be called",
                            )
                        );
                    }
                };

                let mut args = Vec::new();

                while !self.check(&TokenKind::RParen) {
                    if self.is_at_end() {
                        return Err(
                            self.error(
                                "expected `)` after function arguments",
                            )
                        );
                    }

                    args.push(self.argument()?);
                }

                self.consume(
                    &TokenKind::RParen,
                    "expected `)` after function arguments",
                )?;

                expr = Expr::Call {
                    name,
                    args,
                };
            } else if self.matches(&TokenKind::Dot) {
                let name = match self.advance().kind.clone() {
                    TokenKind::Ident(name) => name,
                    _ => {
                        return Err(
                            self.error(
                                "expected field name after `.`",
                            )
                        );
                    }
                };

                expr = Expr::Field {
                    object: Box::new(expr),
                    name,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn argument(&mut self) -> Result<Expr, String> {
        self.term()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.advance().kind.clone() {
            TokenKind::Number(value) => {
                Ok(Expr::Number(value))
            }

            TokenKind::String(value) => {
                Ok(Expr::String(value))
            }

            TokenKind::True => {
                Ok(Expr::Boolean(true))
            }

            TokenKind::False => {
                Ok(Expr::Boolean(false))
            }

            TokenKind::Ident(name) => {
                Ok(Expr::Variable(name))
            }

            TokenKind::Declare => {
                self.consume(
                    &TokenKind::String(String::new()),
                    "expected string after `declare`",
                )?;

                let path = match self.previous().kind.clone() {
                    TokenKind::String(value) => Expr::String(value),
                    _ => unreachable!(),
                };

                Ok(Expr::Declare(Box::new(path)))
            }

            TokenKind::LBracket => {
                let mut elements = Vec::new();

                while !self.check(&TokenKind::RBracket) {
                    if self.is_at_end() {
                        return Err(
                            self.error(
                                "expected `]` after array",
                            )
                        );
                    }

                    elements.push(self.expression()?);
                }

                self.consume(
                    &TokenKind::RBracket,
                    "expected `]` after array",
                )?;

                Ok(Expr::Array(elements))
            }

            TokenKind::Import => {
                self.consume(
                    &TokenKind::LParen,
                    "expected `(` after `import`",
                )?;

                let path = self.expression()?;

                self.consume(
                    &TokenKind::RParen,
                    "expected `)` after import path",
                )?;

                Ok(Expr::Import(Box::new(path)))
            }

            TokenKind::ImportStr => {
                self.consume(
                    &TokenKind::LParen,
                    "expected `(` after `import$str`",
                )?;

                let path = self.expression()?;

                self.consume(
                    &TokenKind::RParen,
                    "expected `)` after import$str path",
                )?;

                Ok(Expr::ImportStr(Box::new(path)))
            }

            TokenKind::LParen => {
                let expr = self.expression()?;

                self.consume(
                    &TokenKind::RParen,
                    "expected `)` after expression",
                )?;

                Ok(expr)
            }

            _ => {
                Err(
                    self.error(
                        "expected expression",
                    )
                )
            }
        }
    }

    fn consume(
        &mut self,
        kind: &TokenKind,
        message: &str,
    ) -> Result<(), String> {
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(message))
        }
    }

    fn matches(
        &mut self,
        kind: &TokenKind,
    ) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn matches_any(
        &mut self,
        kinds: &[TokenKind],
    ) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(
        &self,
        kind: &TokenKind,
    ) -> bool {
        if self.is_at_end() {
            return matches!(kind, TokenKind::Eof);
        }

        token_kind_matches(
            &self.peek().kind,
            kind,
        )
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        matches!(
            self.peek().kind,
            TokenKind::Eof
        )
    }

    fn error(
        &self,
        message: &str,
    ) -> String {
        format!(
            "{message} at token {:?}",
            self.peek()
        )
    }
}

fn token_kind_matches(
    a: &TokenKind,
    b: &TokenKind,
) -> bool {
    matches!(
        (a, b),
        (TokenKind::Var, TokenKind::Var)
            | (TokenKind::Print, TokenKind::Print)
            | (TokenKind::If, TokenKind::If)
            | (TokenKind::Else, TokenKind::Else)
            | (TokenKind::Void, TokenKind::Void)
            | (TokenKind::Return, TokenKind::Return)
            | (TokenKind::True, TokenKind::True)
            | (TokenKind::False, TokenKind::False)
            | (TokenKind::Plus, TokenKind::Plus)
            | (TokenKind::Minus, TokenKind::Minus)
            | (TokenKind::Star, TokenKind::Star)
            | (TokenKind::Slash, TokenKind::Slash)
            | (TokenKind::Percent, TokenKind::Percent)
            | (TokenKind::Equal, TokenKind::Equal)
            | (
                TokenKind::EqualEqual,
                TokenKind::EqualEqual
            )
            | (TokenKind::LParen, TokenKind::LParen)
            | (TokenKind::RParen, TokenKind::RParen)
            | (TokenKind::LBracket, TokenKind::LBracket)
            | (TokenKind::RBracket, TokenKind::RBracket)
            | (TokenKind::LBrace, TokenKind::LBrace)
            | (TokenKind::RBrace, TokenKind::RBrace)
            | (
                TokenKind::Semicolon,
                TokenKind::Semicolon
            )
            | (TokenKind::Eof, TokenKind::Eof)
            | (
                TokenKind::Number(_),
                TokenKind::Number(_)
            )
            | (
                TokenKind::String(_),
                TokenKind::String(_)
            )
            | (
                TokenKind::Ident(_),
                TokenKind::Ident(_)
            )
            | (
                TokenKind::Import,
                TokenKind::Import
            )
            | (
                TokenKind::ImportStr,
                TokenKind::ImportStr
            )
            | (
                TokenKind::Declare,
                TokenKind::Declare
            )
            | (
                TokenKind::Struct,
                TokenKind::Struct
            )
            | (
                TokenKind::Dot,
                TokenKind::Dot
            )
    )
}