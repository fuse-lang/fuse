use fuse_common::Span;
use fuse_common_proc::serializable;
use std::rc::Rc;

#[serializable]
#[derive(Debug)]
pub struct Chunk {
    pub span: Span,
    pub body: Block,
}

#[serializable]
#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn statements(&self) -> &Vec<Statement> {
        self.statements.as_ref()
    }
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// Empty statement for example `;;`
    Empty(EmptyStatement),
    /// An expression statement.
    Expression(Expression),
    /// A variable declaration using const, let or global keywords.
    VariableDeclaration(VariableDeclaration),
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct EmptyStatement {
    pub span: Span,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub span: Span,
    pub kind: VariableDeclarationKind,
    pub binding: BindingPattern,
    pub expression: Option<Expression>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum VariableDeclarationKind {
    Let,
    Const,
    Global,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct BindingPattern {
    pub kind: BindingPatternKind,
    pub type_annotation: Option<TypeAnnotation>,
    pub optional: bool,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum BindingPatternKind {
    Identifier(BindingIdentifier),
    Tuple,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct BindingIdentifier {
    pub span: Span,
    pub atom: Atom,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct TypeAnnotation {}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct Atom(pub Rc<str>);

#[serializable]
#[derive(Debug, PartialEq)]
pub enum Expression {
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    Identifier(Identifier),
    Function(Function),
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct BooleanLiteral {
    pub span: Span,
    pub value: bool,
}

#[serializable]
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

#[serializable]
#[derive(Debug, PartialEq)]
pub enum NumberKind {
    Binary,
    Decimal,
    Hexadecimal,
    Float,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct StringLiteral {
    pub span: Span,
    pub segments: Vec<StringSegment>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum StringSegment {
    Literal(StringLiteralSegment),
    Interpolated(InterpolatedStringSegment),
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum StringLiteralSegment {
    Escaped(Atom),
    Unescaped(Span),
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct InterpolatedStringSegment {
    pub expression: Expression,
    pub format: InterpolationFormat,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum InterpolationFormat {
    Display,
    Debug,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub span: Span,
    pub name: Atom,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct Function {
    pub span: Span,
    pub params: FunctionParameters,
    pub return_type: Option<TypeAnnotation>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct FunctionParameters {
    pub span: Span,
    pub items: Vec<FunctionParameter>,
    pub rest: Option<BindingRest>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct FunctionParameter {
    pub span: Span,
    pub pattern: BindingPattern,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct BindingRest {
    pub span: Span,
    pub binding: BindingIdentifier,
    pub type_annotation: Option<TypeAnnotation>,
}
