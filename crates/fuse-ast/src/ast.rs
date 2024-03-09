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
#[derive(Debug, PartialEq)]
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
    Empty(Box<EmptyStatement>),
    /// An expression statement.
    Expression(Box<Expression>),
    /// A variable declaration using const, let or global keywords.
    VariableDeclaration(Box<VariableDeclaration>),
    /// A function declaration using function or fn keywords.
    FunctionDeclaration(Box<Function>),
    /// An enum declaration using enum keyword.
    EnumDeclaration(Box<EnumDeclaration>),
    /// A struct declaration using struct keyword.
    StructDeclaration(Box<StructDeclaration>),
    /// A struct declaration using struct keyword.
    ImplStatement(Box<ImplStatement>),
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
    pub mutable: bool,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct TypeAnnotation {
    // TODO: at the moment we treat type annotation like identifiers.
    pub identifier: Identifier,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct Atom(pub Rc<str>);

#[serializable]
#[derive(Debug, PartialEq)]
pub enum Expression {
    NumberLiteral(Box<NumberLiteral>),
    StringLiteral(Box<StringLiteral>),
    BooleanLiteral(Box<BooleanLiteral>),
    Identifier(Box<Identifier>),
    Function(Box<Function>),
    If(Box<If>),
    UnaryOperator(Box<UnaryOperator>),
    BinaryOperator(Box<BinaryOperator>),
    ArrayExpression(Box<ArrayExpression>),
    TupleExpression(Box<TupleExpression>),
    ParenthesizedExpression(Box<ParenthesizedExpression>),
    CallExpression(Box<CallExpression>),
    TableConstructionExpression(Box<ConstructionExpression>),
    StructConstructionExpression(Box<StructConstructionExpression>),
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
    pub signature: FunctionSignature,
    pub body: FunctionBody,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct FunctionSignature {
    pub span: Span,
    pub identifier: Option<Identifier>,
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

#[serializable]
#[derive(Debug, PartialEq)]
pub enum FunctionBody {
    Block(Block),
    Expression(Expression),
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct If {
    pub span: Span,
    pub cond: Expression,
    pub body: Block,
    pub r#else: Option<Else>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum Else {
    If(Box<If>),
    Block(Box<Block>),
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct UnaryOperator {
    pub kind: UnaryOperatorKind,
    pub expression: Expression,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum UnaryOperatorKind {
    Not(Span),
    Minus(Span),
    Plus(Span),
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct BinaryOperator {
    pub kind: BinaryOperatorKind,
    pub lhs: Expression,
    pub rhs: Expression,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum BinaryOperatorKind {
    Assignment(Span),
    LogicalOr(Span),
    LogicalAnd(Span),
    BitwiseOr(Span),
    BitwiseXor(Span),
    BitwiseAnd(Span),
    Equality(Span),
    NonEquality(Span),
    LessThanEqual(Span),
    LessThan(Span),
    GreaterThanEqual(Span),
    GreaterThan(Span),
    Plus(Span),
    Minus(Span),
    Multiply(Span),
    Exponential(Span),
    Division(Span),
    FloorDivision(Span),
    Modulo(Span),
    ShiftLeft(Span),
    ShiftRight(Span),
    Member(Span),
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct ArrayExpression {
    pub span: Span,
    pub elements: Vec<ArrayExpressionElement>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum ArrayExpressionElement {
    Expression(Expression),
    Spread(SpreadArgument),
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct TupleExpression {
    pub span: Span,
    pub elements: Vec<TupleExpressionElement>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum TupleExpressionElement {
    Expression(Expression),
    Spread(SpreadArgument),
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct SpreadArgument {
    pub span: Span,
    pub element: Expression,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct KeyValueArgument {
    pub span: Span,
    pub key: Identifier,
    pub value: Expression,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct EnumDeclaration {
    pub span: Span,
    pub identifier: Identifier,
    pub variants: Vec<EnumVariant>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct EnumVariant {
    pub identifier: Identifier,
    pub value: Option<Expression>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct StructDeclaration {
    pub span: Span,
    pub identifier: Identifier,
    pub fields: Vec<StructField>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct StructField {
    pub modifier: VisibilityModifier,
    pub identifier: Identifier,
    pub type_annotation: TypeAnnotation,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum VisibilityModifier {
    Private,
    Public(Span),
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct ParenthesizedExpression {
    pub span: Span,
    pub expression: Expression,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct CallExpression {
    pub span: Span,
    pub target: Expression,
    pub arguments: Vec<Expression>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct StructConstructionExpression {
    pub target: Expression,
    pub construction: ConstructionExpression,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct ConstructionExpression {
    pub span: Span,
    pub fields: Vec<ConstructionField>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum ConstructionField {
    Expression(Expression),
    KeyValueArgument(KeyValueArgument),
    Spread(SpreadArgument),
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct ImplStatement {
    pub span: Span,
    pub target: TypeAnnotation,
    pub r#trait: Option<TypeAnnotation>,
    pub methods: Vec<ImplMethod>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct ImplMethod {
    pub modifier: VisibilityModifier,
    pub function: Function,
}
