use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{BooleanLiteral, Else, Expression, Identifier, If};

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self) -> ParserResult<Expression> {
        let expression = match self.cur_kind() {
            TokenKind::True => {
                let token = self.consume();
                Ok(self.ast.boolean_expression(BooleanLiteral {
                    span: token.span(),
                    value: true,
                }))
            }
            TokenKind::False => {
                let token = self.consume();
                Ok(self.ast.boolean_expression(BooleanLiteral {
                    span: token.span(),
                    value: false,
                }))
            }
            TokenKind::NumberLiteral => {
                let expr = self.parse_number_literal()?;
                Ok(self.ast.number_expression(expr))
            }
            TokenKind::StringLiteral | TokenKind::InterpolatedStringHead => {
                let expr = self.parse_string_literal()?;
                Ok(self.ast.string_expression(expr))
            }
            TokenKind::Identifier => self
                .parse_identifier()
                .map(|id| self.ast.identifier_expression(id)),
            TokenKind::Function | TokenKind::Fn => self.parse_function_expression(),
            TokenKind::If => self.parse_if_expression(),

            TokenKind::Not | TokenKind::Plus | TokenKind::Minus => {
                self.parse_unary_operator_expression()
            }

            _ => Err(Self::unexpected_error(self.cur_token())),
        };
        self.parse_proceding_operator(expression?)
    }

    pub(crate) fn parse_identifier(&mut self) -> ParserResult<Identifier> {
        debug_assert!(self.at(TokenKind::Identifier));
        let token = self.consume();
        let view = self.view_token(*token);
        Ok(Identifier {
            span: token.span(),
            name: self.ast.atom(view),
        })
    }

    pub(crate) fn parse_function_expression(&mut self) -> ParserResult<Expression> {
        self.parse_function(false)
            .map(|func| self.ast.function_expression(func))
    }

    fn parse_if_expression(&mut self) -> ParserResult<Expression> {
        self.parse_if().map(|expr| self.ast.if_expression(expr))
    }

    fn parse_if(&mut self) -> ParserResult<If> {
        debug_assert!(matches!(self.cur_kind(), TokenKind::If | TokenKind::ElseIf));
        let start = self.start_span();
        // Consume the keyword
        self.consume();
        let cond = self.parse_expression()?;
        self.consume_expect(TokenKind::Then)?;
        let body = self.parse_block_while(|kind| {
            println!("IF KIND {kind:?}");
            !matches! {
                kind,
                    | TokenKind::End
                    | TokenKind::Else
                    | TokenKind::ElseIf
            }
        })?;
        let r#else = match self.cur_kind() {
            TokenKind::End => {
                self.consume();
                None
            }
            TokenKind::ElseIf => Some(Else::If(Box::from(self.parse_if()?))),
            TokenKind::Else => {
                self.consume();
                Some(Else::Block(Box::from(self.parse_block()?)))
            }
            _ => return Err(Self::unexpected_error(&self.prev_token)),
        };
        // how to detect end of block?
        // maybe via a predicate function?
        Ok(If {
            span: self.end_span(start),
            cond,
            body,
            r#else,
        })
    }

    pub(crate) fn parse_unary_operator_expression(&mut self) -> ParserResult<Expression> {
        self.parse_unary_operator()
            .map(|op| self.ast.unary_operator_expression(op))
    }
}
