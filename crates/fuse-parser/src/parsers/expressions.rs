use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{Expression, NumberKind, NumberLiteral};
use fuse_common::Span;

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self) -> ParserResult<Expression> {
        if !self.at(TokenKind::NumberLiteral) {
            return Err(self.unexpected_error());
        }

        let token = self.consume();

        Ok(Expression::NumberLiteral(NumberLiteral {
            value: 123f64,
            span: token.span,
            kind: NumberKind::Decimal,
        }))
    }
}
