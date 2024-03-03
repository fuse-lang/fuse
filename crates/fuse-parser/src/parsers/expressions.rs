use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{BooleanLiteral, Expression, Identifier};

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
            TokenKind::Identifier => self.parse_identifier().map(|id| Expression::Identifier(id)),
            _ => Err(self.unexpected_error()),
        }
    }
    pub(crate) fn parse_identifier(&mut self) -> ParserResult<Identifier> {
        let token = self.consume();
        let view = self.view_token(*token);
        Ok(Identifier {
            span: token.span(),
            name: self.ast.atom(view),
        })
    }
}
