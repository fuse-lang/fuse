use fuse_common::Span;

use super::{
    string_data::{StringData, StringValue},
    Lexer, Token, TokenKind, TokenReference,
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
                            expected_hashes,
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
                expected_hashes,
                raw: raw_mod,
            },
        );

        Some(token)
    }

    pub(crate) fn follow_string_interpolation(&mut self, head_data: &StringData<'a>) {
        debug_assert_eq!(
            self.current().kind(),
            TokenKind::RCurly,
            "Invalid string interpolation pattern,\
             Following string interpolation expects\
             the current lexed token to be of `}}` kind."
        );

        let start = self.current().start();

        // Clear the lookahead to avoid any conflict.
        self.lookahead.clear();

        let mut builder = StringBuilder::with_head_ref(head_data);
        let mut kind = TokenKind::Undetermined;
        let data_start = self.source.offset();
        let mut data_end: u32 = 0;

        while let Some(next) = self.source.next_char() {
            match (builder.escape, next) {
                (false, '$') if self.source.peek_char() == Some('{') => {
                    self.source.advance();
                    kind = TokenKind::InterpolatedStringMiddle;
                    // ignore ${` at the end
                    data_end = self.source.offset() - 2;
                }
                (false, c) if c == head_data.quote => {
                    let end = self.source.offset();
                    let terminate = self.string_terminate(head_data.raw, head_data.expected_hashes);

                    if terminate {
                        builder.terminate();
                        kind = TokenKind::InterpolatedStringTail;
                        data_end = end;
                        break;
                    } else {
                        builder.lex(c);
                    }
                }
                _ => builder.lex(next),
            }
        }
        let token = self.create(start, kind);
        self.set_string_data(
            token,
            StringData {
                quote: head_data.quote,
                unicode: head_data.unicode,
                expected_hashes: head_data.expected_hashes,
                raw: head_data.raw,
                terminated: builder.terminated,
                value: builder.build(Span::new(data_start, data_end)),
            },
        );
        let token = TokenReference::with_trivia(
            token,
            self.current().leading_trivia.clone(),
            Vec::default(),
        );
        unsafe {
            self.set_current(token);
        }
    }

    fn promote_to_interpolated_string(&mut self, start: u32, data: StringData<'a>) -> Token {
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

    fn with_head_ref(head_data: &StringData) -> Self {
        Self {
            chars: Vec::new(),
            escape: false,
            escape_whitespace: false,
            has_ever_escaped: false,
            terminated: false,
            raw: head_data.raw,
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
