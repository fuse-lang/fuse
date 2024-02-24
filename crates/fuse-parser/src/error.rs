use crate::{
    lexer::{LexerError, TokenKind, TokenReference},
    Parser,
};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0:?}")]
    LexerError(LexerError),
    #[error("Expected \"{expected:?}\" instead found \"{:?}\" at {token:?}", token.kind())]
    UnexpectedTokenKindError {
        token: TokenReference,
        expected: TokenKind,
    },
    #[error("Invalid number literal error at {0:?}")]
    InvalidNumberLiteralError(TokenReference),
    #[error("Unexpected error at {0:?}")]
    UnexpectedError(TokenReference),
}

impl<'a> Parser<'a> {
    pub(crate) fn unexpected_error(&self) -> Error {
        Error::UnexpectedError(self.prev_token.clone())
    }

    pub(crate) fn unexpect_token_kind_error(&self, expected: TokenKind) -> Error {
        Error::UnexpectedTokenKindError {
            token: self.prev_token.clone(),
            expected,
        }
    }

    pub(crate) fn invalid_number_literal_error(&self) -> Error {
        Error::UnexpectedError(self.prev_token.clone())
    }
}
