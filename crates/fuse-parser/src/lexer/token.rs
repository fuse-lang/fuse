use super::Symbol;
use fuse_common::{Position, Span};

pub struct Token {
    span: Span,
    token_kind: TokenKind,
}

impl Token {
    pub fn new(span: Span, token_kind: TokenKind) -> Self {
        Self { span, token_kind }
    }

    pub fn start_position(&self) -> Position {
        self.span.start
    }

    pub fn end_position(&self) -> Position {
        self.span.end
    }
}

pub struct TokenReference {
    leading_trivia: Vec<Token>,
    token: Token,
    trailing_trivia: Vec<Token>,
}

impl TokenReference {
    pub fn new(leading_trivia: Vec<Token>, token: Token, trailing_trivia: Vec<Token>) -> Self {
        Self {
            leading_trivia,
            token,
            trailing_trivia,
        }
    }

    pub fn is_symbol(&self, symbol: Symbol) -> bool {
        self.token.token_kind == TokenKind::Symbol
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Symbol,
    Identifier,
    StringLiteral,
    NumberLiteral,
    Eof,
}
