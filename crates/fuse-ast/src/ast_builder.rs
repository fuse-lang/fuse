use crate::ast::*;

pub fn block(statements: Vec<Statement>) -> Block {
    Block { statements }
}

pub fn statement() -> Statement {
    Statement {
        statement: StatementVariant::None,
        semicolon: None,
    }
}
