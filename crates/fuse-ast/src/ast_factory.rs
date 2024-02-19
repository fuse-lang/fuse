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

    pub fn empty_statement(&self) -> Statement {
        Statement::Empty
    }

    pub fn declaration_statement(&self, decl: VariableDeclaration) -> Statement {
        Statement::VariableDeclaration(decl)
    }

    pub fn variable_declaration(
        &self,
        kind: VariableDeclarationKind,
        binding: BindingPattern,
    ) -> VariableDeclaration {
        VariableDeclaration {
            span: Span::default(),
            kind,
            binding,
        }
    }
}
