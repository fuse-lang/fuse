use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{VariableDeclaration, VariableDeclarationKind};
use fuse_common::Span;

impl<'a> Parser<'a> {
    pub(crate) fn parse_variable_declaration(
        &mut self,
        start_span: Span,
    ) -> ParserResult<VariableDeclaration> {
        let declaration_kind = match self.cur_kind() {
            TokenKind::Let => VariableDeclarationKind::Let,
            TokenKind::Const => VariableDeclarationKind::Const,
            TokenKind::Global => VariableDeclarationKind::Global,
            _ => return Err(self.unexpected_error()),
        };

        self.consume();

        let ident = self.parse_binding();

        todo!()
        // Ok(self.ast.variable_declaration(declarations))
    }
}
