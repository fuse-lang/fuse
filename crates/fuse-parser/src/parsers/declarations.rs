use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{VariableDeclaration, VariableDeclarationKind};
use fuse_common::Span;

impl<'a> Parser<'a> {
    pub(crate) fn parse_variable_declaration(&mut self) -> ParserResult<VariableDeclaration> {
        let decl_kind = match self.cur_kind() {
            TokenKind::Let => VariableDeclarationKind::Let,
            TokenKind::Const => VariableDeclarationKind::Const,
            TokenKind::Global => VariableDeclarationKind::Global,
            _ => return Err(Self::unexpected_error(self.cur_token())),
        };

        let start = self.start_span();

        self.consume();

        let binding = self.parse_binding()?;
        let expression = if let Some(_) = self.consume_if(TokenKind::Eq) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(self
            .ast
            .variable_declaration(self.end_span(start), decl_kind, binding, expression))
    }
}
