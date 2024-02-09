mod source;
mod symbol;
mod token;
mod whitespace;

pub use symbol::*;
pub use token::*;

use fuse_common::{Span, SpanView};

use source::{Source, SourcePosition};

use std::collections::VecDeque;

pub struct Lexer<'a> {
    source: Source<'a>,
    current_token: Option<LexerResult<TokenReference>>,
    next_token: Option<LexerResult<TokenReference>>,
    lookahead: VecDeque<Lookahead<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            source: Source::new(src),
            current_token: None,
            next_token: None,
            lookahead: VecDeque::with_capacity(1),
        }
    }

    pub fn current(&self) -> Option<&LexerResult<TokenReference>> {
        self.current_token.as_ref()
    }

    pub fn peek(&self) -> Option<&LexerResult<TokenReference>> {
        self.next_token.as_ref()
    }

    pub fn consume(&mut self) -> Option<LexerResult<TokenReference>> {
        let current = self.current_token.take()?;

        self.current_token = self.next_token.take();
        self.next_token = self.next_with_trivia();
        Some(current)
    }

    fn next_with_trivia(&mut self) -> Option<LexerResult<TokenReference>> {
        let mut leading_trivia = Vec::new();
        let mut errors: Option<Vec<LexerError>> = None;

        let token = loop {
            match self.next()? {
                LexerResult::Ok(token) if token.kind().is_trivial() => {
                    leading_trivia.push(token);
                }
                LexerResult::Ok(token) => {
                    break token;
                }
                LexerResult::Fatal(error) => return Some(LexerResult::Fatal(error)),
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
            Some(errors) => Some(LexerResult::Recovered(token, errors)),
            None => Some(LexerResult::Ok(token)),
        }
    }

    fn next(&mut self) -> Option<LexerResult<Token>> {
        let start = self.source.offset();

        let Some(first) = self.source.next_char() else {
            return self.create(start, TokenKind::Eof);
        };

        match first {
            ' ' | '\t' | '\r' => self.whitespace(start, first),
            _ => None,
        }
    }

    fn collect_trailing_trivia(&mut self) -> Vec<Token> {
        let mut trailing_trivia = Vec::new();

        loop {
            let start = self.source.position();

            match self.next() {
                Some(LexerResult::Ok(token)) if token.kind().is_trivial() => {
                    let view = self.view_token(token);
                    trailing_trivia.push(token);
                    if token.kind() == TokenKind::Whitespace && view.contains('\n') {
                        break;
                    }
                }

                _ => {
                    // SAFETY: `start` position is created by `source`,
                    // and it is unchanged from the moment of creation.
                    unsafe { self.source.set_position(start) }
                    break;
                }
            }
        }

        trailing_trivia
    }

    fn create(&self, start: u32, token_kind: TokenKind) -> Option<LexerResult<Token>> {
        Some(LexerResult::Ok(Token::new(
            Span {
                start,
                end: self.source.offset(),
            },
            token_kind,
        )))
    }

    fn view_token(&self, token: Token) -> &'a str {
        self.source.as_str().view(&token.span())
    }
}

pub enum LexerError {}

pub enum LexerResult<T> {
    Ok(T),
    Fatal(Vec<LexerError>),
    Recovered(T, Vec<LexerError>),
}

#[derive(Debug, Clone, Copy)]
struct Lookahead<'a> {
    position: SourcePosition<'a>,
    token: Token,
}
