use super::Symbol;
use fuse_common::Span;

#[derive(Debug, Clone, Copy)]
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
    token: Token,
    leading_trivia: Vec<Token>,
    trailing_trivia: Vec<Token>,
}

impl TokenReference {
    pub fn with_trivia(
        token: Token,
        leading_trivia: Vec<Token>,
        trailing_trivia: Vec<Token>,
    ) -> Self {
        Self {
            token,
            leading_trivia,
            trailing_trivia,
        }
    }

    pub fn is_symbol(&self, symbol: Symbol) -> bool {
        self.token.kind == TokenKind::Symbol && todo!("check the value with {symbol}")
    }
}

// impl std::borrow::Borrow<Token> for &TokenReference {
//     fn borrow(&self) -> &Token {
//         self
//     }
// }

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
    Whitespace,
    Eof,
}

impl TokenKind {
    pub fn is_trivial(&self) -> bool {
        matches! {
            self,
            TokenKind::Whitespace
        }
    }
}
