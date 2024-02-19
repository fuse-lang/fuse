#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Special Tokens
    #[default]
    Eof,
    Undetermined,
    Whitespace,
    Newline,
    Hashbang,

    // Identifiers and Literals
    Identifier,
    StringLiteral,
    NumberLiteral,

    // Symbols
    Let,
    Const,
    Global,

    // Punctuations
    Comma,
    Semicolon,
}

impl TokenKind {
    pub fn is_trivial(&self) -> bool {
        matches!(self, TokenKind::Whitespace)
    }

    pub fn is_symbol(&self) -> bool {
        matches! {
            self,
            | TokenKind::Const
            | TokenKind::Let
        }
    }
}
