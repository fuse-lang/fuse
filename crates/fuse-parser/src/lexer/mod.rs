mod source;
mod symbol;
mod token;

pub use symbol::*;
pub use token::*;

use fuse_common::{Position, Span};

use source::Source;

pub struct Lexer {
    source: Source,
    current_token: Option<LexerResult<TokenReference>>,
    next_token: Option<LexerResult<TokenReference>>,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Self {
            source: Source::new(src),
            current_token: None,
            next_token: None,
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
        let mut errors: Option<Vec<TokenReference>> = None;
    }

    fn next(&mut self) -> Option<LexerResult<Token>> {
        let start = self.source.position();

        let Some(next) = self.source.next() else {
            return self.create(start, TokenKind::Eof);
        };
    }

    fn create(
        &self,
        start_position: Position,
        token_kind: TokenKind,
    ) -> Option<LexerResult<Token>> {
        Some(LexerResult::Ok(Token::new(
            Span {
                start: start_position,
                end: self.source.position(),
            },
            token_kind,
        )))
    }
}

pub enum LexerError {}

pub enum LexerResult<T> {
    Ok(T),
    Fatal(Vec<LexerError>),
    Recovered(T, Vec<LexerError>),
}
