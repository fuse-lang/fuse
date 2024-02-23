use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{VariableDeclaration, VariableDeclarationKind};
use fuse_common::Span;

impl<'a> Parser<'a> {
    pub(crate) fn parse_variable_declaration(
        &mut self,
        start: Span,
    ) -> ParserResult<VariableDeclaration> {
        let decl_kind = match self.cur_kind() {
            TokenKind::Let => VariableDeclarationKind::Let,
            TokenKind::Const => VariableDeclarationKind::Const,
            TokenKind::Global => VariableDeclarationKind::Global,
            _ => return Err(self.unexpected_error()),
        };

        self.consume();

        let binding = self.parse_binding()?;
        let expression = self
            .consume_if(TokenKind::Eq)
            .and_then(|_| self.parse_expression().ok());

        self.ast
            .variable_declaration(decl_kind, binding, expression);
        todo!()
    }
}
