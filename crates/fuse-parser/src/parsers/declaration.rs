use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{VariableDeclaration, VariableDeclarationKind};
use fuse_common::Span;

impl<'a> Parser<'a> {
    pub(crate) fn parse_variable_declaration(
        &mut self,
        start_span: Span,
    ) -> ParserResult<VariableDeclaration> {
        let declaration_kind = match self.cur_kind().unwrap() {
            TokenKind::Let => VariableDeclarationKind::Let,
            TokenKind::Const => VariableDeclarationKind::Const,
            TokenKind::Global => VariableDeclarationKind::Global,
            _ => return ParserResult::Err,
        };

        ParserResult::Err
        // Ok(self.ast.variable_declaration(declarations))
    }
}
