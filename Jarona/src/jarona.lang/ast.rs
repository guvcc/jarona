#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Var {name: String, value: Expr },
    Assign { name: String, value: Expr },
    Print(Expr),
    Expr(Expr),
}


#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Array(Vec<Expr>),
    String(String),

    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    Import(Box<Expr>),
    ImportStr(Box<Expr>),

    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Negate,
    Plus,
}


#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}