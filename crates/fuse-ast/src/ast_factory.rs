use std::rc::Rc;

use crate::ast::*;
use fuse_common::Span;

pub struct AstFactory();
impl AstFactory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn default() -> Self {
        Self()
    }

    pub fn chunk(&self, span: Span, body: Block) -> Chunk {
        Chunk { span, body }
    }

    pub fn empty_block(&self) -> Block {
        Block {
            statements: Vec::default(),
        }
    }

    pub fn block(&self, statements: Vec<Statement>) -> Block {
        Block { statements }
    }

    pub fn empty_statement(&self, span: Span) -> Statement {
        Statement::Empty(Box::from(EmptyStatement { span }))
    }

    pub fn variable_declaration_statement(&self, decl: VariableDeclaration) -> Statement {
        Statement::VariableDeclaration(Box::from(decl))
    }

    pub fn function_declaration_statement(&self, func: Function) -> Statement {
        Statement::FunctionDeclaration(Box::from(func))
    }

    pub fn enum_declaration_statement(&self, decl: EnumDeclaration) -> Statement {
        Statement::EnumDeclaration(Box::from(decl))
    }

    pub fn struct_declaration_statement(&self, decl: StructDeclaration) -> Statement {
        Statement::StructDeclaration(Box::from(decl))
    }

    pub fn impl_statement(&self, r#impl: ImplStatement) -> Statement {
        Statement::ImplStatement(Box::from(r#impl))
    }

    pub fn expression_statement(&self, expr: Expression) -> Statement {
        Statement::Expression(Box::from(expr))
    }

    pub fn variable_declaration(
        &self,
        span: Span,
        kind: VariableDeclarationKind,
        binding: BindingPattern,
        expression: Option<Expression>,
    ) -> VariableDeclaration {
        VariableDeclaration {
            span,
            kind,
            binding,
            expression,
        }
    }

    pub fn function_declaration(&self) {}

    pub fn binding_identifier_pattern(
        &self,
        binding_identifier: BindingIdentifier,
        type_annotation: Option<TypeAnnotation>,
        optional: bool,
    ) -> BindingPattern {
        BindingPattern {
            kind: BindingPatternKind::Identifier(binding_identifier),
            type_annotation,
            optional,
        }
    }

    pub fn binding_identifier(
        &self,
        span: Span,
        identifier: Identifier,
        mutable: bool,
    ) -> BindingIdentifier {
        BindingIdentifier {
            span,
            identifier,
            mutable,
        }
    }

    pub fn atom(&self, value: &str) -> Atom {
        Atom(Rc::from(value))
    }

    pub fn number_literal(
        &self,
        span: Span,
        raw: Atom,
        value: NumberType,
        kind: NumberKind,
    ) -> NumberLiteral {
        NumberLiteral {
            span,
            raw,
            value,
            kind,
        }
    }

    pub fn boolean_literal_expression(&self, literal: BooleanLiteral) -> Expression {
        Expression::BooleanLiteral(Box::from(literal))
    }

    pub fn number_literal_expression(&self, literal: NumberLiteral) -> Expression {
        Expression::NumberLiteral(Box::from(literal))
    }

    pub fn string_literal_expression(&self, literal: StringLiteral) -> Expression {
        Expression::StringLiteral(Box::from(literal))
    }

    pub fn identifier_expression(&self, ident: Identifier) -> Expression {
        Expression::Identifier(Box::from(ident))
    }

    pub fn function_expression(&self, func: Function) -> Expression {
        Expression::Function(Box::from(func))
    }

    pub fn if_expression(&self, expr: If) -> Expression {
        Expression::If(Box::from(expr))
    }

    pub fn unary_operator_expression(&self, op: UnaryOperator) -> Expression {
        Expression::UnaryOperator(Box::from(op))
    }

    pub fn binary_operator_expression(&self, op: BinaryOperator) -> Expression {
        Expression::BinaryOperator(Box::from(op))
    }

    pub fn array_expression(
        &self,
        span: Span,
        elements: Vec<ArrayExpressionElement>,
    ) -> Expression {
        Expression::ArrayExpression(Box::from(ArrayExpression { span, elements }))
    }

    pub fn tuple_expression(
        &self,
        span: Span,
        elements: Vec<TupleExpressionElement>,
    ) -> Expression {
        Expression::TupleExpression(Box::from(TupleExpression { span, elements }))
    }

    pub fn parenthesized_expression(&self, span: Span, expression: Expression) -> Expression {
        Expression::ParenthesizedExpression(Box::from(ParenthesizedExpression { span, expression }))
    }

    pub fn call_expression(
        &self,
        span: Span,
        callee: Expression,
        arguments: Vec<Expression>,
    ) -> Expression {
        Expression::CallExpression(Box::from(CallExpression {
            span,
            callee,
            arguments,
        }))
    }

    pub fn member_expression(
        &self,
        span: Span,
        lhs: MemberExpressionLHS,
        rhs: MemberExpressionRHS,
    ) -> Expression {
        Expression::MemberExpression(Box::from(MemberExpression {
            span,
            lhs: Box::from(lhs),
            rhs: Box::from(rhs),
        }))
    }

    pub fn struct_construction_expression(
        &self,
        target: Expression,
        construction: ConstructionExpression,
    ) -> Expression {
        Expression::StructConstructionExpression(Box::from(StructConstructionExpression {
            target,
            construction,
        }))
    }

    pub fn table_construction_expression(
        &self,
        construction: ConstructionExpression,
    ) -> Expression {
        Expression::TableConstructionExpression(Box::from(construction))
    }

    pub fn construction_expression(
        &self,
        span: Span,
        fields: Vec<ConstructionField>,
    ) -> ConstructionExpression {
        ConstructionExpression { span, fields }
    }

    pub fn key_value_argument(
        &self,
        span: Span,
        key: Identifier,
        value: Expression,
    ) -> KeyValueArgument {
        KeyValueArgument { span, key, value }
    }
}
