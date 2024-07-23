use lalrpop_util::lalrpop_mod;

use crate::ast::{Expr, Opcode};
use std::io;

mod ast;
lalrpop_mod!(grammar);

fn main() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let expr = grammar::ExprParser::new().parse(&buffer).unwrap();
        println!("{}", calc(*expr));
    }
}

fn calc(e: Expr) -> i32 {
    match e {
        Expr::Number(n) => n,
        Expr::Ident(_) => unimplemented!("Variables not working yet"),
        Expr::Op(a, op, b) => match op {
            Opcode::Mul => calc(*a) * calc(*b),
            Opcode::Div => calc(*a) / calc(*b),
            Opcode::Add => calc(*a) + calc(*b),
            Opcode::Sub => calc(*a) - calc(*b),
        },
    }
}
