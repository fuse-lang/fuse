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

pub enum Statement {
    Empty,
}
