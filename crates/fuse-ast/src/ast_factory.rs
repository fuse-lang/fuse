use crate::ast::*;
use fuse_common::Span;

pub struct AstFactory();
impl AstFactory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn default() -> Self {
        Self()
    }

    pub fn chunk(&self, span: Span, body: Block) -> Chunk {
        Chunk { span, body }
    }

    pub fn block(&self) -> Block {
        Block {
            statements: Vec::default(),
        }
    }

    pub fn block_with_statements(&self, statements: Vec<Statement>) -> Block {
        Block { statements }
    }

    pub fn statement(&self) -> Statement {
        Statement {
            statement: StatementVariant::None,
            semicolon: None,
        }
    }
}
