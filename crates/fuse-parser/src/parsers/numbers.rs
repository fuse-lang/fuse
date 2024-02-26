use std::str::FromStr;
use crate::{Parser, ParserResult};
use fuse_ast::{IntType, NumberKind, NumberLiteral, NumberType};

impl<'a> Parser<'a> {
    pub(crate) fn parse_number_literal(&mut self) -> ParserResult<NumberLiteral> {
        let token = self.consume();

        let raw = self.view_token(*token);

        let (value, kind) = match raw.replace('_', "").as_bytes() {
            [b'0', b'x' | b'X', rest @ ..] => {
                // SAFETY: `rest` is just created from `raw`, and is a valid utf8
                let rest = unsafe { std::str::from_utf8_unchecked(rest) };
                self.parse_hexadecimal_number(rest)
            }
            [b'0', b'b' | b'B', rest @ ..] => {
                // SAFETY: `rest` is just created from `raw`, and is a valid utf8
                let rest = unsafe { std::str::from_utf8_unchecked(rest) };
                self.parse_binary_number(rest)
            }
            rest => {
                // SAFETY: `rest` is just created from `raw`, and is a valid utf8
                let rest = unsafe { std::str::from_utf8_unchecked(rest) };
                self.parse_decimal_number(rest)
            }
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
        let float = {
            let dots = str.match_indices('.').count();
            if dots > 1 {
                return Err(self.unexpected_error());
            }
            dots == 1
        };

        let exponent = {
            let mut exponents: Vec<_> = str.match_indices('e').collect();
            match exponents.len() {
                0 => None,
                1 => exponents.pop(),
                _ => return Err(self.unexpected_error()),
            }
        };

        fn parse<T: FromStr>(parser: &Parser, s: &str) -> ParserResult<T> {
            T::from_str(s).map_err(|_| parser.invalid_number_literal_error())
        }

        let value = if let Some((pos, _)) = exponent {
            let decimal = parse::<NumberType>(self, &str[0..pos])?;
            let exponent = parse::<i32>(self, &str[pos + 1..])?;

            decimal * (10f64.powi(exponent)) as NumberType
        } else {
            parse(self, str)?
        };

        Ok((
            value,
            if float {
                NumberKind::Float
            } else {
                NumberKind::Decimal
            },
        ))
    }
}
