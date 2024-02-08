use super::Symbol;
use fuse_common::Span;

pub struct Token {
    span: Span,
    kind: TokenKind,
}

impl Token {
    pub fn new(span: Span, kind: TokenKind) -> Self {
        Self { span, kind }
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

    #[inline]
    pub fn kind(&self) -> TokenKind {
        self.kind
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
        self.token.kind == TokenKind::Symbol && todo!("check the value with {symbol}")
    }
}

impl std::ops::Deref for TokenReference {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Symbol,
    Identifier,
    StringLiteral,
    NumberLiteral,
    Eof,
}
