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

        let mut builder = StringBuilder::new();

        while let Some(next) = self.source.next_char() {
            match (builder.escape, next) {
                // start of an interpolated string segment.
                (false, '$') if self.source.peek_char() == Some('{') => {
                    // Eat the peeked brace
                    self.source.advance();
                    // ignore the `${` at the end
                    let end = self.source.offset() - 2;
                    return Some(self.promote_to_interpolated_string(
                        start,
                        StringData {
                            quote,
                            value: builder.build(Span::new(data_start, end)),
                            terminated: true,
                            unicode: unicode_mod,
                            raw: raw_mod,
                        },
                    ));
                }

                // possible point of string termination.
                (false, c) if c == quote => {
                    let end = self.source.offset();
                    let terminate = self.string_terminate(raw_mod, expected_hashes);

                    if terminate {
                        builder.terminate();
                        data_end = end;
                        break;
                    } else {
                        builder.lex(c);
                    }
                }
                _ => builder.lex(next),
            }
        }

        // if not terminated
        if !builder.terminated {
            println!("Unterminated string literal!");
        }

        let token = self.create(start, TokenKind::StringLiteral);

        self.set_string_data(
            token,
            StringData {
                quote,
                value: builder.build(Span::new(data_start, data_end)),
                terminated: data_end != 0,
                unicode: unicode_mod,
                raw: raw_mod,
            },
        );

        Some(token)
    }

    pub(crate) fn follow_string_interpolation(&mut self) -> Token {
        if self.source.next_char().unwrap() != '}' {
            panic!("Unterminated string interpolation!");
        }
        todo!()
        // match substitution segment.
        // after each middle segment we expect an expression.
    }

    fn promote_to_interpolated_string(&mut self, start: u32, data: StringData) -> Token {
        let token = self.create(start, TokenKind::InterpolatedStringHead);

        self.set_string_data(token, data);
        token
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
}

struct StringBuilder {
    chars: Vec<char>,
    escape: bool,
    escape_whitespace: bool,
    has_ever_escaped: bool,
    terminated: bool,
    raw: bool,
}

impl StringBuilder {
    fn new() -> Self {
        Self {
            chars: Vec::new(),
            escape: false,
            escape_whitespace: false,
            has_ever_escaped: false,
            terminated: false,
            raw: false,
        }
    }

    fn with_raw_mod() -> Self {
        Self {
            chars: Vec::new(),
            escape: false,
            escape_whitespace: false,
            has_ever_escaped: false,
            terminated: false,
            raw: true,
        }
    }

    fn lex(&mut self, c: char) {
        match (self.escape, c) {
            // Skip escaped whitespaces.
            (true, c) if c.is_ascii_whitespace() => {
                self.escape_whitespace = true;
            }

            // Parse escaped character or terminate whitespace escape sequence.
            (true, c) => {
                self.escape = self.raw;
                if !self.escape_whitespace {
                    if let Some(c) = parse_escaped_character(c) {
                        self.push(c);
                    }
                } else {
                    self.escape_whitespace = false;
                    self.push(c);
                }
            }

            (false, '\\') => {
                self.escape = true;
                // reset whitespace escape sequence.
                self.escape_whitespace = false;
                self.has_ever_escaped = true;
            }

            (_, c) => self.push(c),
        }
    }

    fn terminate(&mut self) {
        self.terminated = true
    }

    fn build(self, span: Span) -> StringValue {
        StringValue::new(span, self.chars, self.has_ever_escaped)
    }

    pub(self) fn push(&mut self, c: char) {
        self.chars.push(c)
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
