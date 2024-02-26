use fuse_common::Span;

use super::{Lexer, Token, TokenKind};

pub use string_data::*;

impl<'a> Lexer<'a> {
    pub(super) fn string(&mut self, start: u32, first: char) -> Option<Token> {
        // Return early if we are not peeking a string otherwise determine it's type.
        let expect_hash = match (first, self.source.peek_pair()) {
            ('"' | '\'', _) => 0,
            ('r', Some(pair @ ('#', '\'' | '"' | '#'))) => {
                self.source.advance_n(2);
                let mut hash_count = if pair.1 == '#' { 2 } else { 1 };
                0
            }
            _ => return None,
        };

        let quote = self
            .source
            .next_char()
            .expect("We already peeked this character");

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
                (true, _) => escape = false,

                // terminate string on matching quote.
                (false, c) if c == quote => {
                    if multiline {
                        // Check for triple quotes string termination
                        if matches!(self.source.peek_pair(), Some((p1, p2)) if p1 == p2 && p2 == quote)
                        {
                            // Eat the 2 quotes.
                            self.source.advance_n(2);
                            // Terminate the string.
                            data_end = self.source.offset();
                            break;
                        }
                    } else {
                        // Terminate the string.
                        data_end = self.source.offset();
                        break;
                    }
                }

                (false, '\n' | '\r') if !multiline => break,

                (false, '\\') => escape = true,
                _ => {}
            }
        }

        let terminated = data_end == 0;

        if terminated {
            println!("Unterminated string literal!");
        }

        let token = self.create(start, TokenKind::StringLiteral);

        self.set_string_data(
            token,
            StringData {
                quote,
                data: "TEst Data!!".to_string(),
                span: Span::new(data_start, data_end),
                terminated,
                special_characters: Vec::new(),
            },
        );

        Some(token)
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

    pub enum SpecialCharacter {
        SingleQuote,
        DoubleQuote,
        DollarSign,
        LCurly,
        RCurly,
    }

    pub struct StringData {
        pub quote: char,
        pub data: String,
        pub span: Span,
        pub terminated: bool,
        pub special_characters: Vec<(SpecialCharacter, usize)>,
    }
}
