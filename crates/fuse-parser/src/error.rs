use crate::{
    lexer::{LexerError, TokenKind, TokenReference},
    Parser,
};
use fuse_common_proc::serializable;
use thiserror::Error as ThisError;

#[serializable]
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0:?}")]
    LexerError(LexerError),
    #[error("Expected \"{expected:?}\" instead found \"{:?}\" at {token:?}", token.kind())]
    UnexpectedTokenKindError {
        token: TokenReference,
        expected: TokenKind,
    },
    #[error("{0}")]
    DiagnosisError(DiagnosisError),
    #[error("Invalid number literal error at {0:?}")]
    InvalidNumberLiteralError(TokenReference),
    #[error("Unexpected error at {0:?}")]
    UnexpectedError(TokenReference),
}

#[serializable]
#[derive(ThisError, Debug)]
pub enum DiagnosisError {
    #[error("{0:?}\n{1}")]
    GeneralError(TokenReference, String),
}

impl<'a> Parser<'a> {
    pub(crate) fn unexpected_error(token: &TokenReference) -> Error {
        Error::UnexpectedError(token.clone())
    }

    pub(crate) fn unexpect_token_kind_error(found: &TokenReference, expected: TokenKind) -> Error {
        Error::UnexpectedTokenKindError {
            token: found.clone(),
            expected,
        }
    }

    pub(crate) fn invalid_number_literal_error(token: &TokenReference) -> Error {
        Error::UnexpectedError(token.clone())
    }

    pub(crate) fn diagnosis_general_error(token: &TokenReference, msg: &str) -> Error {
        Error::DiagnosisError(DiagnosisError::GeneralError(token.clone(), msg.to_string()))
    }
}
