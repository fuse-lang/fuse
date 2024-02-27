use fuse_common::Span;

use super::{Lexer, Token, TokenKind};

pub use string_data::*;

impl<'a> Lexer<'a> {
    pub(super) fn string(&mut self, start: u32, first: char) -> Option<Token> {
        // Return early if we are not peeking a string otherwise determine it's type.
        let (unicode_mod, raw_mod) = self.string_modifiers(first)?;

        if unicode_mod {
            self.source.advance();
        }

        let expected_hashes = if raw_mod {
            self.source.advance();
            self.source.advance_while(|c| matches!(c, '#'))
        } else {
            ""
        };

        let quote = if matches!(first, '\'' | '"') {
            first
        } else {
            self.source.next_char()?
        };

        let data_start = self.source.offset();
        let mut data_end = 0;

        let mut has_ever_escaped = false;
        let mut escape = false;
        let mut value = Vec::<char>::new();

        while let Some(next) = self.source.next_char() {
            let accept = match (escape, next) {
                // Accept escaped character no matter what.
                (true, _) => {
                    escape = raw_mod;
                    true
                }

                // terminate string on matching quote.
                (false, c) if c == quote => {
                    let end = self.source.offset();
                    let terminate = if raw_mod {
                        let position = self.source.position();
                        let hashes = self.source.advance_while(|c| c == '#');
                        if hashes == expected_hashes {
                            true
                        } else {
                            // SAFETY: this position is created from the same source,
                            // and source never changes.
                            unsafe {
                                self.source.set_position(position);
                            }
                            false
                        }
                    } else {
                        true
                    };

                    if terminate {
                        data_end = end;
                        break;
                    } else {
                        true
                    }
                }

                (false, '\\') => {
                    escape = true;
                    false
                }
                _ => true,
            };

            if escape {
                has_ever_escaped = true;
            }

            if accept {
                value.push(next);
            }
        }

        // if terminated
        if data_end != 0 {
            println!("Unterminated string literal!");
        }

        let token = self.create(start, TokenKind::StringLiteral);

        let value = if has_ever_escaped {
            // TODO: look into more efficent ways to collect char array into string.
            StringValue::Escaped(value.iter().collect())
        } else {
            StringValue::Unescaped(Span::new(data_start, data_end))
        };

        self.set_string_data(
            token,
            StringData {
                quote,
                value,
                terminated: data_end != 0,
                unicode: unicode_mod,
                interpolations: Vec::new(),
            },
        );

        Some(token)
    }

    fn string_modifiers(&mut self, first: char) -> Option<(bool, bool)> {
        match (first, self.source.peek_pair()) {
            ('"' | '\'', _) => Some((false, false)),
            ('u', Some(('\'' | '"', _))) => Some((true, false)),
            ('r', Some(('#', '\'' | '"' | '#'))) => Some((false, true)),
            ('u', Some(('r', '#'))) => Some((true, true)),
            _ => None,
        }
    }
}

mod string_data {
    use super::{Lexer, Token};
    use fuse_common::Span;
    impl<'a> Lexer<'a> {
        /// Get a reference to the string data related to the given token.
        /// It can panic if token dosn't have any stored string.
        pub fn get_string_data(&mut self, token: &Token) -> &mut StringData {
            self.strings_data.get_mut(&token).unwrap()
        }

        /// Get the ownership of string data related to the given token.
        /// It can panic if token dosn't have any stored string.
        pub fn eat_string_data(&mut self, token: &Token) -> StringData {
            self.strings_data.remove(token).unwrap()
        }

        pub(super) fn set_string_data(&mut self, token: Token, data: StringData) -> bool {
            self.strings_data.insert(token, data).is_some()
        }
    }

    pub struct StringData {
        pub quote: char,
        pub value: StringValue,
        pub terminated: bool,
        pub unicode: bool,
        pub interpolations: Vec<Span>,
    }

    pub enum StringValue {
        Escaped(String),
        Unescaped(Span),
    }
}
