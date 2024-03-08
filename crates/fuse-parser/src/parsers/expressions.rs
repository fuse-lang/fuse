use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{
    ArrayExpressionElement, BinaryOperator, BooleanLiteral, Else, Expression, Identifier, If,
    Precedence, SpreadElement, TupleExpressionElement,
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self) -> ParserResult<Expression> {
        let expr = self.parse_primary_expression()?;
        self.parse_expression_with_precedence(expr, Precedence::Expression)
    }

    pub(crate) fn try_parse_expression(&mut self) -> Option<ParserResult<Expression>> {
        let result = self.try_parse_primary_expression()?;
        Some(
            result.and_then(|expr| {
                self.parse_expression_with_precedence(expr, Precedence::Expression)
            }),
        )
    }

    pub(crate) fn parse_primary_expression(&mut self) -> ParserResult<Expression> {
        if let Some(result) = self.try_parse_primary_expression() {
            result
        } else {
            Err(Self::unexpected_error(self.cur_token()))
        }
    }

    pub(crate) fn try_parse_primary_expression(&mut self) -> Option<ParserResult<Expression>> {
        use TokenKind::*;
        match self.cur_kind() {
            True => {
                let token = self.consume();
                Some(Ok(self.ast.boolean_expression(BooleanLiteral {
                    span: token.span(),
                    value: true,
                })))
            }
            False => {
                let token = self.consume();
                Some(Ok(self.ast.boolean_expression(BooleanLiteral {
                    span: token.span(),
                    value: false,
                })))
            }
            NumberLiteral => Some(
                self.parse_number_literal()
                    .map(|expr| self.ast.number_expression(expr)),
            ),
            StringLiteral | InterpolatedStringHead => Some(
                self.parse_string_literal()
                    .map(|expr| self.ast.string_expression(expr)),
            ),
            Identifier => Some(
                self.parse_identifier()
                    .map(|id| self.ast.identifier_expression(id)),
            ),

            Function | TokenKind::Fn => Some(self.parse_function_expression()),
            If => Some(self.parse_if_expression()),

            Not | Plus | Minus => Some(self.parse_unary_operator_expression()),
            LBrack => Some(self.parse_array_expression()),
            LParen => Some(self.parse_tuple_expression()),

            _ => None,
        }
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

    fn parse_array_expression(&mut self) -> ParserResult<Expression> {
        let start = self.start_span();
        // consume the opening bracket
        self.consume();
        let mut elements: Vec<ArrayExpressionElement> = Vec::new();

        // return early for empty arrays
        if self.consume_if(TokenKind::RBrack).is_some() {
            return Ok(self.ast.array_expression(self.end_span(start), elements));
        }

        loop {
            let element = match self.cur_kind() {
                TokenKind::Dot3 => ArrayExpressionElement::Spread(self.parse_spread_element()?),
                _ => ArrayExpressionElement::Expression(self.parse_expression()?),
            };

            elements.push(element);

            if self.consume_if(TokenKind::Comma).is_none() {
                break;
            }
        }

        self.consume_expect(TokenKind::RBrack)?;

        Ok(self.ast.array_expression(self.end_span(start), elements))
    }

    fn parse_tuple_expression(&mut self) -> ParserResult<Expression> {
        let start = self.start_span();
        // consume the opening parentheses.
        self.consume();
        let mut elements: Vec<TupleExpressionElement> = Vec::new();

        // return early for empty tuples
        if self.consume_if(TokenKind::RParen).is_some() {
            return Ok(self.ast.tuple_expression(self.end_span(start), elements));
        }

        loop {
            let element = match self.cur_kind() {
                TokenKind::Dot3 => TupleExpressionElement::Spread(self.parse_spread_element()?),
                _ => TupleExpressionElement::Expression(self.parse_expression()?),
            };

            elements.push(element);

            if self.consume_if(TokenKind::Comma).is_none() {
                break;
            }
        }

        self.consume_expect(TokenKind::RParen)?;

        Ok(self.ast.tuple_expression(self.end_span(start), elements))
    }

    fn parse_spread_element(&mut self) -> ParserResult<SpreadElement> {
        debug_assert!(self.at(TokenKind::Dot3));
        let start = self.start_span();
        // eat the spread operator.
        self.consume();
        let expression = self.parse_expression()?;
        Ok(SpreadElement {
            span: self.end_span(start),
            element: expression,
        })
    }

    fn parse_expression_with_precedence(
        &mut self,
        lhs: Expression,
        min_precedence: Precedence,
    ) -> ParserResult<Expression> {
        let mut lhs = lhs;
        loop {
            // early return if there is no proceding binary operator.
            let Some(op_precedence) = self.cur_kind().to_precedence() else {
                return Ok(lhs);
            };

            if op_precedence < min_precedence {
                return Ok(lhs);
            }

            let kind = self.parse_binary_operator_kind()?;

            let mut rhs = {
                let Some(rhs) = self.try_parse_primary_expression() else {
                    // TODO: message Expected expression, found `{token}`.
                    self.push_error(Self::unexpected_error(self.cur_token()));
                    return Ok(lhs);
                };
                rhs?
            };

            while let Some(next_precedence) = self.cur_kind().to_precedence() {
                let search_precedence = if next_precedence > op_precedence {
                    op_precedence + 1
                } else if next_precedence.is_right_associative() && next_precedence == op_precedence
                {
                    op_precedence
                } else {
                    break;
                };
                rhs = self.parse_expression_with_precedence(rhs, search_precedence)?;
            }

            lhs = self
                .ast
                .binary_operator_expression(BinaryOperator { kind, lhs, rhs })
        }
    }
}
