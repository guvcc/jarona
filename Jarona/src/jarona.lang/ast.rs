#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Var {name: String, value: Expr },
    Assign { name: String, value: Expr },
    Print(Expr),
    If { condition: Expr, body: Vec<Stmt>, else_body: Option<Vec<Stmt>> },
    Void { name: String, params: Vec<String>, body: Vec<Stmt>},
    Expr(Expr),
    Return(Expr),
    Struct { name: String, fields: Vec<String> },
    Enum { name: String, variants: Vec<String>}
}


#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Array(Vec<Expr>),
    String(String),
    Boolean(bool),
    
    EnumValue {
        enum_name: String,
        variant: String,
    },

    Field {
        object: Box<Expr>,
        name: String,
    },

    Call {
        name: String,
        args: Vec<Expr>,
    },

    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    Import(Box<Expr>),
    ImportStr(Box<Expr>),
    Declare(Box<Expr>),

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
    Equal,
    Modulo,
}