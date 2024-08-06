#[derive(Debug)]
pub enum Statement {
    Print(Box<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Float(f32),
    Ident(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}
