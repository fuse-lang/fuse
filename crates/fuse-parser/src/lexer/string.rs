use fuse_common::Span;

use super::{
    string_data::{StringData, StringValue},
    Lexer, Token, TokenKind,
};

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

        let value = self.collect_string_literal(quote, raw_mod, expected_hashes);

                // // start of an interpolated string segment.
                // (false, '$') if self.source.peek_char() == Some('{') => {
                //     // Eat the peeked brace
                //     self.source.advance();
                //     // ignore the `${` at the end
                //     let end = self.source.offset() - 2;
                //     return Some(self.promote_to_interpolated_string(
                //         start,
                //         StringData {
                //             quote,
                //             value: StringValue::new(
                //                 Span::new(data_start, end),
                //                 value,
                //                 has_ever_escaped,
                //             ),
                //             terminated: true,
                //             unicode: unicode_mod,
                //             raw: raw_mod,
                //         },
                //     ));
                // }

        // if not terminated
        if data_end == 0 {
            println!("Unterminated string literal!, {value:?}");
        }

        let token = self.create(start, TokenKind::StringLiteral);

        self.set_string_data(
            token,
            StringData {
                quote,
                value: StringValue::new(Span::new(data_start, data_end), value, has_ever_escaped),
                terminated: data_end != 0,
                unicode: unicode_mod,
                raw: raw_mod,
            },
        );

        Some(token)
    }

    fn string_modifiers(&mut self, first: char) -> Option<(bool, bool)> {
        let res = match (first, self.source.peek_pair()) {
            ('"' | '\'', _) => Some((false, false)),
            ('u', Some(('\'' | '"', _))) => Some((true, false)),
            ('r', Some(('#', '\'' | '"' | '#'))) => Some((false, true)),
            ('u', Some(('r', '#'))) => Some((true, true)),
            _ => None,
        };
        res
    }

    fn string_terminate(&mut self, raw_mod: bool, expected_hashes: &str) -> bool {
        if raw_mod {
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
        }
    }

    fn collect_string_literal(&self, quote: char, raw_mod: bool, expected_hashes: &str) -> Vec<char> {
        let mut value = Vec::new();
        let mut escape = raw_mod;
        let mut escape_whitespace = false;

        let mut has_ever_escaped = false;
        let mut data_end = 0;

        while let Some(next) = self.source.next_char() {
            let c = match (escape, next) {
                // Skip escaped whitespaces.
                (true, c) if c.is_ascii_whitespace() => {
                    escape_whitespace = true;
                    None
                }
                // Parse escape character or terminate space escape.
                (true, c) => {
                    escape = raw_mod;
                    if !escape_whitespace {
                        parse_escaped_character(c)
                    } else {
                        escape_whitespace = false;
                        Some(c)
                    }
                }

                // possible point of string termination.
                (false, c) if c == quote => {
                    let end = self.source.offset();
                    let terminate = self.string_terminate(raw_mod, expected_hashes);

                    if terminate {
                        data_end = end;
                        break;
                    } else {
                        Some(c)
                    }
                }


                (false, '\\') => {
                    escape = true;
                    escape_whitespace = false;
                    None
                }

                (_, c) => Some(c),
            };

            if escape {
                has_ever_escaped = true;
            }

            if let Some(c) = c {
                value.push(c);
            }
        }
        value
    }
}

fn parse_escaped_character(c: char) -> Option<char> {
    match c {
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        '\\' => Some('\\'),
        '0' => Some('\0'),
        '$' => Some('$'),
        _ => None,
    }
}
