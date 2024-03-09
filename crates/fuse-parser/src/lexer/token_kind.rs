use fuse_ast::Precedence;
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
    Local,
    Match,
    Mut,
    Never,
    Nil,
    Not,
    Pub,
    Or,
    Own,
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
    /// .
    Dot,
    /// ..
    Dot2,
    /// ...
    Dot3,
    /// ,
    Comma,
    /// :
    Colon,
    /// ;
    Semicolon,
    /// (
    LParen,
    /// )
    RParen,
    /// {
    LCurly,
    /// }
    RCurly,
    /// [
    LBrack,
    /// ]
    RBrack,
    /// <
    LAngle,
    /// >
    RAngle,
    /// <=
    LtEq,
    /// >=
    GtEq,
    /// =
    Eq,
    /// ==
    Eq2,
    /// !=
    Neq,
    /// -
    Minus,
    /// +
    Plus,
    /// *
    Star,
    /// **
    Star2,
    /// /
    Slash,
    /// //
    Slash2,
    /// %
    Percent,
    /// &
    Amp,
    /// ^
    Caret,
    /// |
    Pipe,
    /// <<
    LShift,
    /// >>
    RShift,
    /// =>
    Arrow,
    /// ->
    ThinArrow,
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

    pub fn to_precedence(self) -> Option<Precedence> {
        use Precedence::*;
        use TokenKind::*;
        match self {
            Eq => Some(Assignment),
            Or => Some(LogicalOr),
            And => Some(LogicalAnd),
            Pipe => Some(BitwiseOr),
            Caret => Some(BitwiseXor),
            Amp => Some(BitwiseAnd),
            Eq2 | Neq => Some(Equality),
            LAngle | RAngle | LtEq | GtEq | As | In => Some(Relational),
            LShift | RShift => Some(Shift),
            Plus | Minus => Some(Add),
            Star | Slash | Slash2 | Percent => Some(Multiply),
            Star2 => Some(Exponential),
            Dot => Some(Member),
            _ => None,
        }
    }
}
