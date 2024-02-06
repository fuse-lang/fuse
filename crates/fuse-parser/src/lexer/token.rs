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

pub enum TokenKind {
    Symbol,
    Identifier,
    StringLiteral,
    NumberLiteral,
    Eof,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub(crate) bytes: usize,
    pub(crate) line: usize,
    pub(crate) character: usize,
}

impl Position {
    pub fn new(bytes: usize, line: usize, character: usize) -> Self {
        Self {
            bytes,
            line,
            character,
        }
    }

    pub fn bytes(self) -> usize {
        self.bytes
    }

    pub fn line(self) -> usize {
        self.line
    }

    pub fn character(self) -> usize {
        self.character
    }
}
