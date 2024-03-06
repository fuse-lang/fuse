use fuse_ast::{
    BinaryOperator, BinaryOperatorKind, Expression, Precedence, UnaryOperator, UnaryOperatorKind,
};

use crate::{
    lexer::{Token, TokenKind},
    Parser, ParserResult,
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_unary_operator(&mut self) -> ParserResult<UnaryOperator> {
        match self.cur_kind() {
            TokenKind::Not => self.parse_unary_not_operator(),
            TokenKind::Plus => self.parse_unary_plus_operator(),
            TokenKind::Minus => self.parse_unary_minus_operator(),
            _ => Err(Self::unexpected_error(self.cur_token())),
        }
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

    pub(crate) fn parse_binary_operator_kind(&mut self) -> ParserResult<BinaryOperatorKind> {
        use TokenKind::*;
        let token = self.consume();
        macro_rules! match_op {
            { $($kind:ident => $op:ident)+ } => (
                match *token {
                    $(Token { kind: $kind, span } => Ok(BinaryOperatorKind::$op(span)),)+
                    _ => Err(Self::unexpected_error(&token)),
                }
            )
        }
        match_op! {
            Or => LogicalOr
            And => LogicalAnd
            Pipe => BitwiseOr
            Amp => BitwiseAnd
            Eq2 => Equality
            Neq => NonEquality
            LAngle => LessThan
            RAngle => GreaterThan
            LtEq => LessThanEqual
            GtEq => GreaterThanEqual
            Plus => Plus
            Minus => Minus
            Star => Multiply
            Slash => Division
            Slash2 => FloorDivision
            Percent => Modulo
        }
    }
}
