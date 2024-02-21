use crate::{
    lexer::{LexerError, TokenReference},
    Parser,
};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0:?}")]
    LexerError(LexerError),
    #[error("Unexpected {0:?}")]
    UnexpectedError(TokenReference),
}

impl<'a> Parser<'a> {
    pub(crate) fn unexpected_error(&self) -> Error {
        let token = self.cur_token();
        Error::UnexpectedError(token.clone())
    }
}
