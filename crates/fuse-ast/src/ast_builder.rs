use crate::ast::*;

pub fn block(statements: Vec<(Statement, Semicolon)>) -> Block {
    Block { statements }
}
