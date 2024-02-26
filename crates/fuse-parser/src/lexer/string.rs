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

        let quote = self.source.next_char()?;

        let multiline = match self.source.peek_pair() {
            // Detect multi-line string from its triple quote.
            Some((p1, p2)) if p1 == p2 && p2 == quote => {
                // Eat the 2 peeked chars.
                self.source.advance_n(2);
                true
            }
            _ => false,
        };

        let data_start = self.source.offset();
        let mut data_end = 0;

        let mut escape = false;

        // panic!("{:?}", self.current());

        while let Some(next) = self.source.next_char() {
            match (escape, next) {
                // Accept escaped character no matter what.
                (true, _) => escape = raw_mod,

                // terminate string on matching quote.
                (false, c) if c == quote => {
                    let end = if multiline {
                        // Check for triple quotes string termination
                        if matches!(self.source.peek_pair(), Some((p1, p2)) if p1 == p2 && p2 == quote)
                        {
                            // Eat the 2 quotes.
                            self.source.advance_n(2);
                            // Terminate the string.
                            self.source.offset()
                        } else {
                            0
                        }
                    } else {
                        // Terminate the string.
                        self.source.offset()
                    };

                    let terminated = if end != 0 {
                        if raw_mod {
                            let hashes = self.source.advance_while(|c| c == '#');
                            if hashes == expected_hashes {
                                true
                            } else {
                                false
                            }
                        } else {
                            true
                        }
                    } else {
                        false
                    };

                    if terminated {
                        data_end = end;
                        break;
                    }
                }

                (false, '\n' | '\r') if !multiline => break,

                (false, '\\') => escape = true,
                _ => {}
            }
        }

        // if terminated
        if data_end != 0 {
            println!("Unterminated string literal!");
        }

        let token = self.create(start, TokenKind::StringLiteral);

        self.set_string_data(
            token,
            StringData {
                quote,
                data: "TEst Data!!".to_string(),
                span: Span::new(data_start, data_end),
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
        pub fn get_string_data(&'a self, token: &Token) -> &'a StringData {
            &self.strings_data[&token]
        }

        pub(super) fn set_string_data(&mut self, token: Token, data: StringData) -> bool {
            self.strings_data.insert(token, data).is_some()
        }
    }

    pub struct StringData {
        pub quote: char,
        pub data: String,
        pub span: Span,
        pub terminated: bool,
        pub unicode: bool,
        pub interpolations: Vec<Span>,
    }
}
