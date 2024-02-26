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
        Statement::Empty(EmptyStatement { span })
    }

    pub fn declaration_statement(&self, decl: VariableDeclaration) -> Statement {
        Statement::VariableDeclaration(decl)
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

    pub fn binding_identifier(&self, span: Span, atom: Atom) -> BindingIdentifier {
        BindingIdentifier { span, atom }
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
}
