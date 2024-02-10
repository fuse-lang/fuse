use fuse_common::Span;

pub struct Chunk {
    pub span: Span,
    pub body: Block,
}

pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn statements(&self) -> &Vec<Statement> {
        self.statements.as_ref()
    }
}

pub struct Statement {
    pub statement: StatementVariant,
    pub semicolon: Option<Semicolon>,
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
    Empty,
}

pub struct Semicolon {
    pub span: Span,
}
