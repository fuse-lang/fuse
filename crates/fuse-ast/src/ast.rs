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

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    /// Empty statement for example `;;`
    Empty,
    /// A variable declaration using const, let or global keywords.
    VariableDeclaration(VariableDeclaration),
}

#[derive(Debug, PartialEq, Eq)]
pub struct VariableDeclaration {
    pub span: Span,
    pub kind: VariableDeclarationKind,
    pub binding: BindingPattern,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VariableDeclarationKind {
    Let,
    Const,
    Global,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BindingPattern {
    pub kind: BindingPatternKind,
    pub type_annotation: Option<TypeAnnotation>,
    pub optional: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BindingPatternKind {
    Identifier,
    Tuple,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeAnnotation {}
