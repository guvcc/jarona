#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Var,
    Print,
    Ident(String),
    Number(f64),
    String(String),
    Import,
    ImportStr,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Eof,
    Semicolon,

}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub pos: usize,
}