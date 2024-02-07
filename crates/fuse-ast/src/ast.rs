pub struct Chunk {

}

pub struct Block {
    pub(crate) statements: Vec<Statement>,
}

impl Block {
    pub fn statements(&self) -> &Vec<Statement> {
        self.statements.as_ref()
    }
}

pub enum Statement {}

pub struct Semicolon {}
