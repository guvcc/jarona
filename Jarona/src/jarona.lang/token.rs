#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Var,
    Print,
    Ident(String),
    Number(f64),
    Import,
    ImportStr,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    LParen,
    RParen,
    Eof,
    Semicolon,

}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    pub kind: TokenKind,
    pub lexeme: String,
    pub pos: usize,
}