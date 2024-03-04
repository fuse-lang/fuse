mod flash_match;
mod identifier;
mod keyword;
mod number;
mod operator;
mod source;
mod string;
mod token;
mod token_kind;
mod whitespace;

pub use string_data::{StringData, StringValue};
pub use token::*;
pub use token_kind::*;

use fuse_common::{Span, SpanView, debug_println};
use fuse_common_proc::serializable;

use source::{Source, SourcePosition};

use std::collections::{HashMap, VecDeque};

pub struct Lexer<'a> {
    source: Source<'a>,
    current_token: TokenReference,
    lookahead: VecDeque<Lookahead<'a>>,
    strings_data: HashMap<Token, StringData<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut lexer = Self {
            source: Source::new(src),
            current_token: TokenReference::default(),
            lookahead: VecDeque::new(),
            strings_data: HashMap::new(),
        };

        // Consume the default token to load the first set of tokens.
        lexer.consume();
        lexer
    }

    pub fn current(&self) -> &TokenReference {
        debug_assert!(
            self.source.offset() != 0 || self.source.is_eof(),
            "attempt to access `current` before advancing to the first token."
        );

        &self.current_token
    }

    pub fn lookahead(&mut self, n: u8) -> &TokenReference {
        // Cache the new lookahead if it dosn't exists.
        self.ensure_lookahead(n);

        &self.lookahead[n as usize - 1].token
    }

    pub fn consume(&mut self) -> TokenReference {
        let next = match self.lookahead.pop_front() {
            Some(next) => {
                // SAFETY: all lookaheads belong to this lexer
                // and `self.source` never changes.
                unsafe { self.source.set_position(next.position) };
                next.token
            }
            None => self.next_with_trivia(),
        };

        // SAFETY: `next` is either aqquired through `self.lookahead`
        // or the `self.next_with_trivia()`, either way it is created,
        // using the same lexer as `self.current_token`.
        let current = unsafe { self.set_current(next) };

        // ensure the existence of at least one lookahead.
        // self.ensure_lookahead(1);

        current
    }

    /// Replace the current token and returns the old one back.
    /// SAFETY: `token` should be a token created using the `self`,
    /// and have the same lifetime and alignment.
    unsafe fn set_current(&mut self, token: TokenReference) -> TokenReference {
        // SAFETY: Both the current and next token are created by this
        // `Lexer` and have the same lifetime and alignment.
        let current = std::ptr::read(&mut self.current_token);
        std::ptr::write(&mut self.current_token, token);
        current
    }

    fn ensure_lookahead(&mut self, n: u8) {
        let n = n as usize;
        debug_assert!(n > 0);

        // Save the initial position.
        let position = self.source.position();

        // Move the source head to the position at the last lookahead state.
        if let Some(lookahead) = self.lookahead.back() {
            // SAFETY: We never change the `self.source` and `self.lookahead`s
            // are all created in this `Lexer` instance and all belong to the same `Source`.
            unsafe { self.source.set_position(lookahead.position) };
        }

        for _ in self.lookahead.len()..n {
            let next = self.next_with_trivia();
            self.lookahead.push_back(Lookahead {
                position: self.source.position(),
                token: next,
            });
        }

        // SAFETY: Position is created at the begining of the function,
        // and `self.source` dosn't change throughout the `Lexer`'s lifetime.
        // Restore the source to the initial `position`.
        unsafe { self.source.set_position(position) };
    }

    fn next_with_trivia(&mut self) -> TokenReference {
        let mut leading_trivia = Vec::new();
        let mut errors: Option<Vec<LexerError>> = None;

        let token = loop {
            let next = self.next();
            if next.kind().is_trivial() {
                leading_trivia.push(next);
            } else {
                break next;
            }
        };

        let trailing_trivia = self.collect_trailing_trivia();
        TokenReference::with_trivia(token, leading_trivia, trailing_trivia)
    }

    fn next(&mut self) -> Token {
        let start = self.source.offset();

        if self.source.is_eof() {
            return self.create(start, TokenKind::Eof);
        }
        let Some(first) = self.source.next_char() else {
            return self.create(start, TokenKind::Eof);
        };

        macro_rules! analyze {
            {$(|)? $lex1:ident $(| $lexn:ident)*} => {
                if let Some(token) = self.$lex1(start, first) {token}
                $(else if let Some(token) = self.$lexn(start, first) {token})+
                else {
                    let token = self.create(start, TokenKind::Undetermined);
                    debug_println!("Undetermined token found while lexing,\n\
                                    Token: {token:?},\n\
                                    Value: {}", self.view_token(token));
                    token
                }
            };
        }

        analyze! {
            | whitespace
            | keyword
            | operator
            | identifier
            | number
            | string
        }
    }

    fn collect_trailing_trivia(&mut self) -> Vec<Token> {
        let mut trailing_trivia = Vec::new();

        loop {
            let start = self.source.position();

            match self.next() {
                token if token.kind().is_trivial() => {
                    let view = self.view_token(token);
                    trailing_trivia.push(token);
                    if token.kind() == TokenKind::Whitespace && view.contains('\n') {
                        break;
                    }
                }

                _ => {
                    // SAFETY: The `start` position is created by `source`,
                    // and it is unchanged from the moment of creation.
                    unsafe { self.source.set_position(start) }
                    break;
                }
            }
        }

        trailing_trivia
    }

    fn create(&self, start: u32, token_kind: TokenKind) -> Token {
        Token::new(
            Span {
                start,
                end: self.source.offset(),
            },
            token_kind,
        )
    }

    pub(super) fn view_token(&self, token: Token) -> &'a str {
        self.source.as_str().view(&token.span())
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = TokenReference;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.consume();
        if token.kind == TokenKind::Eof {
            None
        } else {
            Some(token)
        }
    }
}

mod string_data {
    use super::{Lexer, Token};
    use fuse_common::Span;
    impl<'a> Lexer<'a> {
        /// Get a reference to the string data related to the given token.
        /// It can panic if token dosn't have any stored string.
        pub fn get_string_data(&mut self, token: &Token) -> &'a mut StringData {
            self.strings_data.get_mut(&token).unwrap()
        }

        /// Get the ownership of string data related to the given token.
        /// It can panic if token dosn't have any stored string.
        pub fn eat_string_data(&mut self, token: &Token) -> StringData<'a> {
            self.strings_data.remove(token).unwrap()
        }

        /// Set the string data for the given token.
        /// It returns `None` if the key has no previous value,
        /// otherwise returns the `Some` of old value.
        ///
        /// Internal
        pub(super) fn set_string_data(
            &mut self,
            token: Token,
            data: StringData<'a>,
        ) -> Option<StringData<'a>> {
            self.strings_data.insert(token, data)
        }
    }

    pub struct StringData<'a> {
        pub quote: char,
        pub value: StringValue,
        pub terminated: bool,
        pub unicode: bool,
        pub expected_hashes: &'a str,
        pub raw: bool,
    }

    pub enum StringValue {
        Escaped(String),
        Unescaped(Span),
    }

    impl StringValue {
        pub(super) fn new(span: Span, value: Vec<char>, escaped: bool) -> Self {
            if escaped {
                // TODO: look into more efficent ways to collect char array into string.
                Self::Escaped(value.into_iter().collect())
            } else {
                Self::Unescaped(span)
            }
        }
    }
}

#[serializable]
#[derive(Debug)]
pub enum LexerError {}

#[derive(Debug)]
struct Lookahead<'a> {
    position: SourcePosition<'a>,
    token: TokenReference,
}
