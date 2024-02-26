use super::{Lexer, Token, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn number(&mut self, start: u32, first: char) -> Option<Token> {
        if !first.is_ascii_digit() {
            return None;
        }

        if first == '0' {
            match self.source.peek_char() {
                Some('x' | 'X') => {
                    self.source.advance();
                    self.eat_hexadecimal_literal();
                }
                Some('b' | 'B') => {
                    self.source.advance();
                    self.eat_binary_literal();
                }
                _ => self.eat_decimal_or_float_literal(),
            }
        } else {
            self.eat_decimal_or_float_literal()
        }

        Some(self.create(start, TokenKind::NumberLiteral))
    }

    fn eat_hexadecimal_literal(&mut self) {
        self.source.advance_while(|next| {
            matches! {
                next,
                | '0'..='9'
                | 'a'..='f'
                | 'A'..='F'
                | '_'
            }
        });
    }

    fn eat_binary_literal(&mut self) {
        self.source.advance_while(|next| {
            matches! {
                next,
                | '0'
                | '1'
                | '_'
            }
        });
    }

    /// Eats a decimal or floating point number literal,
    /// And returns the `NumberKind` corresponding to it.
    fn eat_decimal_or_float_literal(&mut self) {
        let mut met_dot = false;
        let mut met_exponent = false;
        let mut met_exponent_sign = false;
        self.source.advance_while_mut(move |next| {
            match next {
                // Don't accept any dots if we already consumed one.
                '.' if met_dot => false,
                '.' => {
                    met_dot = true;
                    true
                }
                // Don't accept any exponent if we already consumed one.
                'e' if met_exponent => false,
                'e' => {
                    met_exponent = true;
                    true
                }
                '-' | '+' if met_exponent_sign => false,
                '-' | '+' => {
                    met_exponent_sign = true;
                    true
                }
                '0'..='9' => true,
                '_' => true,
                _ => false,
            }
        });
    }
}
