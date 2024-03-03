use fuse_common_proc::serializable;

#[serializable]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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
    NumberLiteral,
    StringLiteral,
    InterpolatedStringHead,
    InterpolatedStringMiddle,
    InterpolatedStringTail,

    // Symbols
    And,
    As,
    Break,
    Const,
    Do,
    Else,
    ElseIf,
    End,
    Enum,
    Export,
    False,
    For,
    From,
    Function,
    Fn,
    Global,
    If,
    Impl,
    Import,
    In,
    Let,
    Match,
    Never,
    Nil,
    Not,
    Pub,
    Or,
    Repeat,
    Return,
    /// Lower case `self`
    LowSelf,
    /// Capital case `Self`
    CapSelf,
    Static,
    Struct,
    Then,
    Trait,
    True,
    Type,
    Union,
    Unknown,
    Until,
    Unsafe,
    When,
    While,

    // Punctuations
    Comma,
    Semicolon,
    LParen,
    RParen,
    LCurly,
    RCurly,
    Eq,
    Minus,
    Plus,
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

    pub fn is_valid_identifier(&self) -> bool {
        matches!(self, TokenKind::Identifier) && !self.is_reserved_keyword()
    }

    pub fn is_reserved_keyword(&self) -> bool {
        use TokenKind::*;
        matches! {
            self,
            | And
            | As
            | Break
            | Const
            | Do
            | Else
            | ElseIf
            | End
            | Enum
            | Export
            | False
            | For
            | From
            | Function
            | Fn
            | Global
            | If
            | Impl
            | Import
            | In
            | Let
            | Match
            | Never
            | Nil
            | Not
            | Pub
            | Or
            | Repeat
            | Return
            | LowSelf
            | CapSelf
            | Static
            | Struct
            | Then
            | Trait
            | True
            | Type
            | Union
            | Unknown
            | Until
            | Unsafe
            | When
            | While
        }
    }
}
