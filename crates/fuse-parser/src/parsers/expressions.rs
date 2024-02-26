use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::Expression;

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self) -> ParserResult<Expression> {
        match self.cur_kind() {
            TokenKind::NumberLiteral => Ok(Expression::NumberLiteral(self.parse_number_literal()?)),
            TokenKind::StringLiteral => Ok(Expression::StringLiteral(self.parse_string_literal()?)),
            _ => Err(self.unexpected_error()),
        }
    }
}
