use fuse_ast::{Expression, UnaryOperator, UnaryOperatorKind};

use crate::{lexer::TokenKind, Parser, ParserResult};

impl<'a> Parser<'a> {
    pub(crate) fn parse_unary_operator(&mut self) -> ParserResult<UnaryOperator> {
        match self.cur_kind() {
            TokenKind::Not => self.parse_unary_not_operator(),
            TokenKind::Plus => self.parse_unary_plus_operator(),
            TokenKind::Minus => self.parse_unary_minus_operator(),
            _ => Err(Self::unexpected_error(self.cur_token())),
        }
    }

    pub(crate) fn parse_proceding_operator(&mut self, lhs: Expression) -> ParserResult<Expression> {
        self.parse_proceding_operator_recursive(lhs)
    }

    fn parse_unary_not_operator(&mut self) -> ParserResult<UnaryOperator> {
        debug_assert!(self.at(TokenKind::Not));
        // Consume the keyword.
        let op = self.consume();
        Ok(UnaryOperator {
            kind: UnaryOperatorKind::Not(op.span()),
            expression: self.parse_expression()?,
        })
    }

    fn parse_unary_plus_operator(&mut self) -> ParserResult<UnaryOperator> {
        debug_assert!(self.at(TokenKind::Plus));
        // Consume the keyword.
        let op = self.consume();
        Ok(UnaryOperator {
            kind: UnaryOperatorKind::Plus(op.span()),
            expression: self.parse_expression()?,
        })
    }

    fn parse_unary_minus_operator(&mut self) -> ParserResult<UnaryOperator> {
        debug_assert!(self.at(TokenKind::Minus));
        // Consume the keyword.
        let op = self.consume();
        Ok(UnaryOperator {
            kind: UnaryOperatorKind::Minus(op.span()),
            expression: self.parse_expression()?,
        })
    }

    fn parse_proceding_operator_recursive(&mut self, lhs: Expression) -> ParserResult<Expression> {
        // early return if there is no proceding binary operator.
        if !self.cur_kind().is_binary_operator() {
            return Ok(lhs);
        }
        todo!()
    }
}
