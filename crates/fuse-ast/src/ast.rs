use fuse_common::Span;
use std::rc::Rc;

#[derive(Debug)]
pub struct Chunk {
    pub span: Span,
    pub body: Block,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn statements(&self) -> &Vec<Statement> {
        self.statements.as_ref()
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    /// Empty statement for example `;;`
    Empty(EmptyStatement),
    /// A variable declaration using const, let or global keywords.
    VariableDeclaration(VariableDeclaration),
}

#[derive(Debug, PartialEq)]
pub struct EmptyStatement {
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub span: Span,
    pub kind: VariableDeclarationKind,
    pub binding: BindingPattern,
    pub expression: Option<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum VariableDeclarationKind {
    Let,
    Const,
    Global,
}

#[derive(Debug, PartialEq)]
pub struct BindingPattern {
    pub kind: BindingPatternKind,
    pub type_annotation: Option<TypeAnnotation>,
    pub optional: bool,
}

#[derive(Debug, PartialEq)]
pub enum BindingPatternKind {
    Identifier(BindingIdentifier),
    Tuple,
}

#[derive(Debug, PartialEq)]
pub struct BindingIdentifier {
    pub span: Span,
    pub atom: Atom,
}

#[derive(Debug, PartialEq)]
pub struct TypeAnnotation {}

#[derive(Debug, PartialEq)]
pub struct Atom(pub Rc<str>);

#[derive(Debug, PartialEq)]
pub enum Expression {
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
}

#[derive(Debug, PartialEq)]
pub struct NumberLiteral {
    pub span: Span,
    /// Raw value in the source code.
    pub raw: Atom,
    pub value: NumberType,
    pub kind: NumberKind,
}

pub type NumberType = f64;
pub type IntType = i64;

#[derive(Debug, PartialEq)]
pub enum NumberKind {
    Binary,
    Decimal,
    Hexadecimal,
    Float,
}

#[derive(Debug, PartialEq)]
pub struct StringLiteral {
    pub span: Span,
    pub segments: Vec<StringSegment>,
}

#[derive(Debug, PartialEq)]
pub enum StringSegment {
    Literal(StringLiteralSegment),
    Interpolated(InterpolatedStringSegment),
}

#[derive(Debug, PartialEq)]
pub enum StringLiteralSegment {
    Escaped(Atom),
    Unescaped(Span),
}

#[derive(Debug, PartialEq)]
pub struct InterpolatedStringSegment {
    pub span: Span,
    pub expression: Expression,
    pub format: InterpolationFormat,
}

#[derive(Debug, PartialEq)]
pub enum InterpolationFormat {
    Display,
    Debug,
}
