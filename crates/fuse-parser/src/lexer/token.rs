use super::TokenKind;
use fuse_common::Span;
use fuse_common_proc::serializable;

#[serializable]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
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

#[serializable]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct TokenReference {
    pub token: Token,
    pub leading_trivia: Vec<Token>,
    pub trailing_trivia: Vec<Token>,
}

impl TokenReference {
    pub(crate) fn new(token: Token) -> Self {
        Self::with_trivia(token, Vec::default(), Vec::default())
    }

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
