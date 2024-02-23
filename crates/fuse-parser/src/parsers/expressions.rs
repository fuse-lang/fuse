use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::Expression;
use fuse_common::Span;

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self) -> ParserResult<Expression> {
        todo!()
    }
}
