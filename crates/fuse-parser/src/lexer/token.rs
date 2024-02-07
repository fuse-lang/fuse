use super::Symbol;
use fuse_common::Position;

pub struct Token {
    start_position: Position,
    end_position: Position,
    token_kind: TokenKind,
}

impl Token {
    pub fn new(start_position: Position, end_position: Position, token_kind: TokenKind) -> Self {
        Self {
            start_position,
            end_position,
            token_kind,
        }
    }

    pub fn start_position(&self) -> Position {
        self.start_position
    }

    pub fn end_position(&self) -> Position {
        self.end_position
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
