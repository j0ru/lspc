#[derive(Debug)]
pub enum Expr {
    Number(i32),
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
