use fuse_common::{ReferenceType, Span};
use fuse_common_proc::serializable;
use std::{cell::Cell, rc::Rc};

use crate::GetSpan;

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
    pub identifier: Identifier,
    pub mutable: bool,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct TypeAnnotation {
    // TODO: at the moment we treat type annotation like identifiers.
    pub identifier: Identifier,
}

#[serializable]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Atom(pub Rc<str>);

impl Atom {
    pub fn as_str<'a>(&'a self) -> &'a str {
        &self.0
    }
}

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
    MemberExpression(Box<MemberExpression>),
    CallExpression(Box<CallExpression>),
    TableConstructionExpression(Box<ConstructionExpression>),
    StructConstructionExpression(Box<StructConstructionExpression>),
}

impl Expression {
    pub fn span(&self) -> Span {
        use Expression::*;
        match self {
            NumberLiteral(expr) => expr.span,
            StringLiteral(expr) => expr.span,
            BooleanLiteral(expr) => expr.span,
            Identifier(expr) => expr.span,
            Function(expr) => expr.span,
            If(expr) => expr.span,
            UnaryOperator(expr) => expr.span(),
            BinaryOperator(expr) => expr.span(),
            ArrayExpression(expr) => expr.span,
            TupleExpression(expr) => expr.span,
            ParenthesizedExpression(expr) => expr.span,
            MemberExpression(expr) => expr.span,
            CallExpression(expr) => expr.span,
            TableConstructionExpression(expr) => expr.span,
            StructConstructionExpression(expr) => expr.span(),
        }
    }
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
    pub reference: Cell<Option<ReferenceType>>,
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

impl GetSpan for UnaryOperator {
    #[inline]
    fn span(&self) -> Span {
        Span::with_spans(vec![self.kind.span(), self.expression.span()])
    }
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum UnaryOperatorKind {
    Not(Span),
    Plus(Span),
    Minus(Span),
}

impl GetSpan for UnaryOperatorKind {
    fn span(&self) -> Span {
        match self {
            Self::Not(span) => *span,
            Self::Plus(span) => *span,
            Self::Minus(span) => *span,
        }
    }
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct BinaryOperator {
    pub kind: BinaryOperatorKind,
    pub lhs: Expression,
    pub rhs: Expression,
}

impl GetSpan for BinaryOperator {
    fn span(&self) -> Span {
        Span::with_spans(vec![self.kind.span(), self.lhs.span(), self.rhs.span()])
    }
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

impl GetSpan for BinaryOperatorKind {
    fn span(&self) -> Span {
        let span = match self {
            Self::Assignment(span) => span,
            Self::LogicalOr(span) => span,
            Self::LogicalAnd(span) => span,
            Self::BitwiseOr(span) => span,
            Self::BitwiseXor(span) => span,
            Self::BitwiseAnd(span) => span,
            Self::Equality(span) => span,
            Self::NonEquality(span) => span,
            Self::LessThanEqual(span) => span,
            Self::LessThan(span) => span,
            Self::GreaterThanEqual(span) => span,
            Self::GreaterThan(span) => span,
            Self::Plus(span) => span,
            Self::Minus(span) => span,
            Self::Multiply(span) => span,
            Self::Exponential(span) => span,
            Self::Division(span) => span,
            Self::FloorDivision(span) => span,
            Self::Modulo(span) => span,
            Self::ShiftLeft(span) => span,
            Self::ShiftRight(span) => span,
            Self::Member(span) => span,
        };
        *span
    }
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
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct MemberExpression {
    pub span: Span,
    pub lhs: Box<MemberExpressionLHS>,
    pub rhs: Box<MemberExpressionRHS>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum MemberExpressionLHS {
    Identifier(Identifier),
    Expression(Expression),
    Member(MemberExpression),
    Call(CallExpression),
}

impl From<MemberExpressionRHS> for MemberExpressionLHS {
    fn from(value: MemberExpressionRHS) -> Self {
        match value {
            MemberExpressionRHS::Number(num) => {
                Self::Expression(Expression::NumberLiteral(Box::from(num)))
            }
            MemberExpressionRHS::Identifier(ident) => Self::Identifier(ident),
            MemberExpressionRHS::Member(member) => Self::Member(member),
            MemberExpressionRHS::Call(call) => Self::Call(call),
        }
    }
}

#[serializable]
#[derive(Debug, PartialEq)]
pub enum MemberExpressionRHS {
    Identifier(Identifier),
    Number(NumberLiteral),
    Member(MemberExpression),
    Call(CallExpression),
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct StructConstructionExpression {
    pub target: Expression,
    pub construction: ConstructionExpression,
}

impl GetSpan for StructConstructionExpression {
    fn span(&self) -> Span {
        Span::with_spans(vec![self.target.span(), self.construction.span])
    }
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
    pub r#trait: TypeAnnotation,
    pub methods: Vec<ImplMethod>,
}

#[serializable]
#[derive(Debug, PartialEq)]
pub struct ImplMethod {
    pub modifier: VisibilityModifier,
    pub function: Function,
}
