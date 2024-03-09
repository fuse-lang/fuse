use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{
    ArrayExpressionElement, BinaryOperator, BooleanLiteral, ConstructionExpression,
    ConstructionField, Else, Expression, Identifier, If, KeyValueArgument, Precedence,
    SpreadArgument, TupleExpressionElement,
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self) -> ParserResult<Expression> {
        let expr = self.parse_primary_expression()?;
        self.parse_expression_with_precedence(expr, Precedence::Expression)
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
        let expr = match self.cur_kind() {
            True => {
                let token = self.consume();
                Ok(self.ast.boolean_expression(BooleanLiteral {
                    span: token.span(),
                    value: true,
                }))
            }
            False => {
                let token = self.consume();
                Ok(self.ast.boolean_expression(BooleanLiteral {
                    span: token.span(),
                    value: false,
                }))
            }
            NumberLiteral => self
                .parse_number_literal()
                .map(|expr| self.ast.number_expression(expr)),
            StringLiteral | InterpolatedStringHead => self
                .parse_string_literal()
                .map(|expr| self.ast.string_expression(expr)),
            Identifier => self
                .parse_identifier()
                .map(|id| self.ast.identifier_expression(id)),

            Function | TokenKind::Fn => self.parse_function_expression(),
            If => self.parse_if_expression(),

            Not | Plus | Minus => self.parse_unary_operator_expression(),
            LBrack => self.parse_array_expression(),
            LParen => self.parse_tuple_or_parenthesized_expression(),
            LCurly => self.parse_table_construction_expression(),

            _ => return None,
        };

        let Ok(expr) = expr else {
            return Some(expr);
        };

        Some(self.parse_expression_with_suffix(expr))
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

        // return early for empty arrays
        if self.consume_if(TokenKind::RBrack).is_some() {
            return Ok(self
                .ast
                .array_expression(self.end_span(start), Vec::default()));
        }

        let (elements, _) =
            self.parse_comma_seperated_expressions(|parser| match parser.cur_kind() {
                TokenKind::Dot3 => Some(
                    parser
                        .parse_spread_element()
                        .map(|expr| ArrayExpressionElement::Spread(expr)),
                ),
                _ => Some(
                    parser
                        .parse_expression()
                        .map(|expr| ArrayExpressionElement::Expression(expr)),
                ),
            })?;

        self.consume_expect(TokenKind::RBrack)?;

        Ok(self.ast.array_expression(self.end_span(start), elements))
    }

    fn parse_tuple_or_parenthesized_expression(&mut self) -> ParserResult<Expression> {
        let start = self.start_span();
        // consume the opening parentheses.
        self.consume();
        let mut elements: Vec<TupleExpressionElement> = Vec::new();

        // return early for empty tuples
        if self.consume_if(TokenKind::RParen).is_some() {
            return Ok(self.ast.tuple_expression(self.end_span(start), elements));
        }

        let mut met_comma = false;
        while !self.at(TokenKind::RParen) {
            let element = match self.cur_kind() {
                TokenKind::Dot3 => TupleExpressionElement::Spread(self.parse_spread_element()?),
                _ => TupleExpressionElement::Expression(self.parse_expression()?),
            };

            elements.push(element);

            if self.consume_if(TokenKind::Comma).is_none() {
                break;
            } else {
                met_comma = true;
            }
        }

        self.consume();

        let span = self.end_span(start);
        if met_comma {
            Ok(self.ast.tuple_expression(span, elements))
        } else {
            debug_assert_eq!(elements.len(), 1);
            match elements[0] {
                TupleExpressionElement::Spread(..) => {
                    // accept parenthesized spread expressions as tuples.
                    Ok(self.ast.tuple_expression(span, elements))
                }
                TupleExpressionElement::Expression(..) => {
                    let TupleExpressionElement::Expression(expr) = elements.remove(0) else {
                        unreachable!("Enum variant already got checked");
                    };
                    Ok(self.ast.parenthesized_expression(span, expr))
                }
            }
        }
    }

    fn parse_spread_element(&mut self) -> ParserResult<SpreadArgument> {
        debug_assert!(self.at(TokenKind::Dot3));
        let start = self.start_span();
        // eat the spread operator.
        self.consume();
        let expression = self.parse_expression()?;
        Ok(SpreadArgument {
            span: self.end_span(start),
            element: expression,
        })
    }

    fn parse_comma_seperated_expressions<R, F: Fn(&mut Parser<'a>) -> Option<ParserResult<R>>>(
        &mut self,
        parser: F,
    ) -> ParserResult<(Vec<R>, bool)> {
        let mut expressions = Vec::new();
        let mut met_comma = false;

        while let Some(expr) = parser(self) {
            expressions.push(expr?);
            if self.consume_if(TokenKind::Comma).is_none() {
                break;
            } else {
                met_comma = true;
            }
        }

        Ok((expressions, met_comma))
    }

    fn parse_expression_with_suffix(&mut self, expr: Expression) -> ParserResult<Expression> {
        if !matches!(
            expr,
            Expression::Identifier(..) | Expression::ParenthesizedExpression(..)
        ) {
            return Ok(expr);
        }

        match self.cur_kind() {
            TokenKind::LCurly => self.parse_struct_construction_expression(expr),
            TokenKind::LParen => self.parse_call_expression(expr),
            _ => Ok(expr),
        }
    }

    fn parse_struct_construction_expression(
        &mut self,
        target: Expression,
    ) -> ParserResult<Expression> {
        let construction = self.parse_construction_expression()?;
        Ok(self
            .ast
            .struct_construction_expression(target, construction))
    }

    fn parse_table_construction_expression(&mut self) -> ParserResult<Expression> {
        self.parse_construction_expression()
            .map(|expr| self.ast.table_construction_expression(expr))
    }

    fn parse_construction_expression(&mut self) -> ParserResult<ConstructionExpression> {
        debug_assert!(self.at(TokenKind::LCurly));
        let start = self.start_span();
        // consume openning curly
        self.consume();

        // return early for construction with no field.
        if self.consume_if(TokenKind::RCurly).is_some() {
            return Ok(self
                .ast
                .construction_expression(self.end_span(start), Vec::default()));
        }

        let (fields, _) = self
            .parse_comma_seperated_expressions(|parser| Some(parser.parse_construction_field()))?;

        self.consume_expect(TokenKind::RCurly)?;

        Ok(self
            .ast
            .construction_expression(self.end_span(start), fields))
    }

    fn parse_construction_field(&mut self) -> ParserResult<ConstructionField> {
        match self.cur_kind() {
            // key value argument
            TokenKind::Identifier if self.nth_kind(1) == TokenKind::Colon => self
                .parse_key_value_argument()
                .map(|kv| ConstructionField::KeyValueArgument(kv)),
            TokenKind::Dot3 => self
                .parse_spread_element()
                .map(|spread| ConstructionField::Spread(spread)),
            _ => self
                .parse_expression()
                .map(|expr| ConstructionField::Expression(expr)),
        }
    }

    fn parse_key_value_argument(&mut self) -> ParserResult<KeyValueArgument> {
        let start = self.start_span();
        let identifier = self.parse_identifier()?;
        // consume colon
        self.consume();
        let expression = self.parse_expression()?;

        Ok(self
            .ast
            .key_value_argument(self.end_span(start), identifier, expression))
    }

    fn parse_call_expression(&mut self, expr: Expression) -> ParserResult<Expression> {
        let start = self.start_span();
        // consume the open parentheses
        self.consume();

        // return early for calls with no arguments.
        if self.consume_if(TokenKind::RParen).is_some() {
            return Ok(self
                .ast
                .call_expression(self.end_span(start), expr, Vec::default()));
        }

        let (arguments, _) =
            self.parse_comma_seperated_expressions(|parser| Some(parser.parse_expression()))?;

        self.consume_expect(TokenKind::RParen)?;

        Ok(self
            .ast
            .call_expression(self.end_span(start), expr, arguments))
    }

    fn parse_expression_with_precedence(
        &mut self,
        expr: Expression,
        min_precedence: Precedence,
    ) -> ParserResult<Expression> {
        let mut lhs = expr;
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
