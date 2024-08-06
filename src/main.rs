use lalrpop_util::lalrpop_mod;

use crate::ast::{Expr, Opcode, Statement};
use std::io;

mod ast;
mod environment;

lalrpop_mod!(grammar);

fn main() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let program = grammar::ProgramParser::new().parse(&buffer).unwrap();
        run_program(program);
    }
}

fn run_program(input: Vec<Box<Statement>>) {
    for statement in input {
        match *statement {
            Statement::Print(expr) => println!("{}", calc(*expr)),
        }
    }
}

fn calc(e: Expr) -> f32 {
    match e {
        Expr::Number(n) => n as f32,
        Expr::Float(n) => n,
        Expr::Ident(_) => unimplemented!("Variables not working yet"),
        Expr::Op(a, op, b) => match op {
            Opcode::Mul => calc(*a) * calc(*b),
            Opcode::Div => calc(*a) / calc(*b),
            Opcode::Add => calc(*a) + calc(*b),
            Opcode::Sub => calc(*a) - calc(*b),
        },
    }
}
