use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{BooleanLiteral, Expression};

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self) -> ParserResult<Expression> {
        match self.cur_kind() {
            TokenKind::True => Ok(Expression::BooleanLiteral(BooleanLiteral {
                span: self.consume().span(),
                value: true,
            })),
            TokenKind::False => Ok(Expression::BooleanLiteral(BooleanLiteral {
                span: self.consume().span(),
                value: false,
            })),
            TokenKind::NumberLiteral => Ok(Expression::NumberLiteral(self.parse_number_literal()?)),
            TokenKind::StringLiteral | TokenKind::InterpolatedStringHead => {
                Ok(Expression::StringLiteral(self.parse_string_literal()?))
            }
            _ => Err(self.unexpected_error()),
        }
    }
}
