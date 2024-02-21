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
    /// Empty statement for example `;;`
    Empty,
    /// A variable declaration using const, let or global keywords.
    VariableDeclaration(VariableDeclaration),
}

pub struct VariableDeclaration {
    pub span: Span,
    pub kind: VariableDeclarationKind,
    pub binding: BindingPattern,
}

pub enum VariableDeclarationKind {
    Let,
    Const,
    Global,
}

pub struct BindingPattern {
    pub kind: BindingPatternKind,
    pub type_annotation: Option<TypeAnnotation>,
    pub optional: bool,
}

pub enum BindingPatternKind {
    Identifier,
    Tuple,
}

pub struct TypeAnnotation {}
