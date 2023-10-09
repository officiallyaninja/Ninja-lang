use crate::{
    ast::{self, ASTNode},
    lex::Operation,
};

fn operate(left: i64, op: Operation, right: i64) -> i64 {
    match op {
        Operation::Add => left + right,
        Operation::Sub => left - right,
        Operation::Div => left / right,
        Operation::Mul => left * right,
    }
}
pub fn parse(tokens: Vec<ASTNode>) -> Option<i64> {
    todo!()
}
