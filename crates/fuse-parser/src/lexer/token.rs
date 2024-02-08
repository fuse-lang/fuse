use super::Symbol;
use fuse_common::Span;

pub struct Token {
    span: Span,
    token_kind: TokenKind,
}

impl Token {
    pub fn new(span: Span, token_kind: TokenKind) -> Self {
        Self { span, token_kind }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn start(&self) -> u32 {
        self.span.start
    }

    pub fn end(&self) -> u32 {
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
        self.token.token_kind == TokenKind::Symbol && todo!()
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
