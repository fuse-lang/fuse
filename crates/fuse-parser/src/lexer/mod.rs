mod flash_match;
mod identifier;
mod keyword;
mod operator;
mod source;
mod token;
mod token_kind;
mod whitespace;

pub use token::*;
pub use token_kind::*;

use fuse_common::{Span, SpanView};

use source::{Source, SourcePosition};

use std::collections::VecDeque;

pub struct Lexer<'a> {
    source: Source<'a>,
    current_token: TokenReference,
    lookahead: VecDeque<Lookahead<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut lexer = Self {
            source: Source::new(src),
            current_token: TokenReference::default(),
            lookahead: VecDeque::new(),
        };

        // Consume the default token to load the first set of tokens.
        lexer.consume();
        lexer
    }

    pub fn current(&self) -> &TokenReference {
        debug_assert!(
            self.source.offset() != 0,
            "attempt to access `current` before advancing to the first token."
        );

        &self.current_token
    }

    pub fn peek(&self) -> Option<&TokenReference> {
        self.lookahead.front().map(|next| &next.result)
    }

    pub fn lookahead(&mut self, n: u8) -> &TokenReference {
        // Cache the new lookahead if it dosn't exists.
        self.ensure_lookahead(n);

        &self.lookahead[n as usize - 1].result
    }

    pub fn consume(&mut self) -> TokenReference {
        let next = match self.lookahead.pop_front() {
            Some(next) => next.result,
            None => self.next_with_trivia(),
        };

        // SAFETY: Both the current and next token are created by this
        // `Lexer` and have the same lifetime and alignment.
        let current = unsafe {
            let current = std::ptr::read(&mut self.current_token);
            std::ptr::write(&mut self.current_token, next);
            current
        };

        // ensure the existence of at least one lookahead.
        self.ensure_lookahead(1);

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
                result: next,
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

        let Some(first) = self.source.next_char() else {
            return self.create(start, TokenKind::Eof);
        };

        macro_rules! analyze {
            {| $first:ident $(| $lexer:ident)*} => {
                if let Some(token) = self.$first(start, first) {token}
                $(else if let Some(token) = self.$lexer(start, first) {token})+
                else { self.create(start, TokenKind::Undetermined) }
            };
        }

        analyze! {
            | whitespace
            | keyword
            | operator
            | identifier
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

#[derive(Debug)]
pub enum LexerError {}

#[derive(Debug)]
struct Lookahead<'a> {
    position: SourcePosition<'a>,
    result: TokenReference,
}
