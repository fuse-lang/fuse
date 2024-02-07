use fuse_common::Span;

pub struct Chunk {}

pub struct Block {
    pub(crate) statements: Vec<Statement>,
}

impl Block {
    pub fn statements(&self) -> &Vec<Statement> {
        self.statements.as_ref()
    }
}

pub struct Statement {
    pub(crate) statement: StatementVariant,
    pub(crate) semicolon: Option<Semicolon>,
}

impl Statement {
    pub fn statement(&self) -> &StatementVariant {
        &self.statement
    }

    pub fn semicolon(&self) -> Option<&Semicolon> {
        self.semicolon.as_ref()
    }
}

pub enum StatementVariant {
    None,
}

pub struct Semicolon {
    pub(crate) span: Span,
}
