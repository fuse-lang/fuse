use crate::{
    lexer::{Token, TokenKind, TokenReference},
    Parser, ParserResult,
};
use fuse_ast::{Expression, IntType, NumberKind, NumberLiteral, NumberType};

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self) -> ParserResult<Expression> {
        if !self.at(TokenKind::NumberLiteral) {
            return Err(self.unexpect_token_kind_error(TokenKind::NumberLiteral));
        }

        let token = self.consume();

        Ok(Expression::NumberLiteral(self.parse_number_literal(token)?))
    }

    fn parse_number_literal(&self, token: TokenReference) -> ParserResult<NumberLiteral> {
        let raw = self.view_token(*token);
        let trimmed_string = raw.replace('_', "");

        let (value, kind) = match raw.as_bytes() {
            &[b'0', b'x' | b'X'] => self.parse_hexadecimal_number(trimmed_string.as_str()),
            &[b'0', b'b' | b'B'] => self.parse_binary_number(trimmed_string.as_str()),
            _ => self.parse_decimal_number(trimmed_string.as_str()),
        }?;

        let atom = self.ast.atom(raw);
        Ok(self.ast.number_literal(token.span, atom, value, kind))
    }

    fn parse_hexadecimal_number(&self, str: &str) -> ParserResult<(NumberType, NumberKind)> {
        let value =
            IntType::from_str_radix(str, 16).map_err(|_| self.invalid_number_literal_error())?;
        Ok((value as NumberType, NumberKind::Hexadecimal))
    }

    fn parse_binary_number(&self, str: &str) -> ParserResult<(NumberType, NumberKind)> {
        let value =
            IntType::from_str_radix(str, 2).map_err(|_| self.invalid_number_literal_error())?;
        Ok((value as NumberType, NumberKind::Binary))
    }

    fn parse_decimal_number(&self, str: &str) -> ParserResult<(NumberType, NumberKind)> {
        let dots: Vec<_> = str.match_indices('.').collect();
        if dots.len() > 1 {
            Err(self.unexpected_error())
        } else {
            let value = str::parse(str).map_err(|_| self.invalid_number_literal_error())?;
            Ok((
                value,
                if dots.len() == 0 {
                    NumberKind::Decimal
                } else {
                    NumberKind::Float
                },
            ))
        }
    }
}
