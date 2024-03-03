use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{BooleanLiteral, Expression, Function, FunctionParameters, Identifier};

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
            TokenKind::Function | TokenKind::Fn => self
                .parse_function_expression()
                .map(|func| Expression::Function(func)),
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

    pub(crate) fn parse_function_expression(&mut self) -> ParserResult<Function> {
        let start = self.start_span();
        // Consume the keyword
        self.consume();
        self.consume_expect(TokenKind::RParen)?;
        Ok(Function {
            span: self.end_span(start),
            params: FunctionParameters {
                span: todo!(),
                items: todo!(),
                rest: todo!(),
            },
        })
    }
}
