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
        // let mut leading_trivia = Vec::new();
        // let mut errors: Option<Vec<TokenReference>> = None;
        todo!()
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

    fn create(&self, start: u32, token_kind: TokenKind) -> Option<LexerResult<Token>> {
        Some(LexerResult::Ok(Token::new(
            Span {
                start,
                end: self.source.offset(),
            },
            token_kind,
        )))
    }

    fn view_token(&self, token: TokenReference) -> &'a str {
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
