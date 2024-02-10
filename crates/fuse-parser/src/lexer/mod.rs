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
    current_token: LexerResult<TokenReference>,
    lookahead: VecDeque<Lookahead<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut lexer = Self {
            source: Source::new(src),
            current_token: LexerResult::default(),
            lookahead: VecDeque::new(),
        };

        // Consume the default token to load the first set of tokens.
        lexer.consume();
        lexer
    }

    pub fn current(&self) -> &LexerResult<TokenReference> {
        debug_assert!(
            self.source.offset() == 0,
            "attempt to access `current` before advancing to the first token."
        );

        &self.current_token
    }

    pub fn peek(&self) -> Option<&LexerResult<TokenReference>> {
        self.lookahead.front().map(|next| &next.result)
    }

    pub fn lookahead(&mut self, n: u8) -> &LexerResult<TokenReference> {
        // Cache the new lookahead if it dosn't exists.
        self.ensure_lookahead(n);

        &self.lookahead[n as usize - 1].result
    }

    pub fn consume(&mut self) -> LexerResult<TokenReference> {
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

        // ensure existence of at least one lookahead.
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

    fn next_with_trivia(&mut self) -> LexerResult<TokenReference> {
        let mut leading_trivia = Vec::new();
        let mut errors: Option<Vec<LexerError>> = None;

        let token = loop {
            match self.next() {
                LexerResult::Ok(token) if token.kind().is_trivial() => {
                    leading_trivia.push(token);
                }

                LexerResult::Ok(token) => {
                    break token;
                }

                LexerResult::Fatal(error) => return LexerResult::Fatal(error),

                LexerResult::Recovered(token, mut token_errors) => {
                    match errors.as_mut() {
                        Some(errors) => errors.append(&mut token_errors),
                        _ => errors = Some(token_errors),
                    }

                    if token.kind().is_trivial() {
                        leading_trivia.push(token);
                    } else {
                        break token;
                    }
                }
            }
        };

        let trailing_trivia = self.collect_trailing_trivia();
        let token = TokenReference::with_trivia(token, leading_trivia, trailing_trivia);

        match errors {
            Some(errors) => LexerResult::Recovered(token, errors),
            None => LexerResult::Ok(token),
        }
    }

    fn next(&mut self) -> LexerResult<Token> {
        let start = self.source.offset();

        let Some(first) = self.source.next_char() else {
            return self.create(start, TokenKind::Eof);
        };

        match first {
            ' ' | '\t' | '\r' => self.whitespace(start, first),
            _ => unreachable!("lexer couldn't tokenize the character {first} at {start} position"),
        }
    }

    fn collect_trailing_trivia(&mut self) -> Vec<Token> {
        let mut trailing_trivia = Vec::new();

        loop {
            let start = self.source.position();

            match self.next() {
                LexerResult::Ok(token) if token.kind().is_trivial() => {
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

    fn create(&self, start: u32, token_kind: TokenKind) -> LexerResult<Token> {
        LexerResult::Ok(Token::new(
            Span {
                start,
                end: self.source.offset(),
            },
            token_kind,
        ))
    }

    fn view_token(&self, token: Token) -> &'a str {
        self.source.as_str().view(&token.span())
    }
}

#[derive(Debug)]
pub enum LexerError {}

#[derive(Debug)]
pub enum LexerResult<T> {
    Ok(T),
    Fatal(Vec<LexerError>),
    Recovered(T, Vec<LexerError>),
}

impl Default for LexerResult<TokenReference> {
    fn default() -> Self {
        Self::Ok(TokenReference::new(Token::default()))
    }
}

#[derive(Debug)]
struct Lookahead<'a> {
    position: SourcePosition<'a>,
    result: LexerResult<TokenReference>,
}
