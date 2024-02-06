#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

macro_rules! def {
    {
        $(#[$meta:meta])*
        enum {
            $(
                $name:ident => $value:literal,
            )+
        }
    } => {
        paste::paste! {
            $(#[$meta])*
            pub enum Symbol {
                $(
                    #[cfg_attr(feature = "serde", serde(rename = $string))]
                    $name,
                )+
            }

            impl Symbol {
                pub fn from_str(symbol: &str) -> Option<Self> {
                    match symbol {
                        $(
                            $value => Some(Self::$name),
                        )+
                        _ => None,
                    }
                }
            }

            impl Display for Symbol {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    match self {
                        $(
                            Self::$name => f.write_str($value),
                        )+
                    }
                }
            }
        }
    };
}

def! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
    enum {
        And => "and",
        Break => "break",
        Const => "const",
        Do => "do",
        Else => "else",
        ElseIf => "elseif",
        End => "end",
        False => "false",
        For => "for",
        Function => "function",
        If => "if",
        In => "in",
        Let => "let",
        Nil => "nil",
        Not => "not",
        Or => "or",
        Repeat => "repeat",
        Return => "return",
        Then => "then",
        True => "true",
        Until => "until",
        While => "while",
        PlusEqual => "+=",
        MinusEqual => "-=",
        StarEqual => "*=",
        SlashEqual => "/=",
        DoubleSlashEqual => "//=",
        PercentEqual => "%=",
        CaretEqual => "^=",
        Ampersand => "&",
        ThinArrow => "->",
        TwoColons => "::",
        Caret => "^",
        Colon => ":",
        Comma => ",",
        Dot => ".",
        TwoDots => "..",
        Ellipse => "...",
        Equal => "=",
        TwoEqual => "==",
        GreaterThan => ">",
        GreaterThanEqual => ">=",
        DoubleGreaterThan => ">>",
        Hash => "#",
        LeftBrace => "{",
        LeftBracket => "[",
        LeftParen => "(",
        LessThan => "<",
        LessThanEqual => "<=",
        DoubleLessThan => "<<",
        Minus => "-",
        Percent => "%",
        Pipe => "|",
        Plus => "+",
        QuestionMark => "?",
        RightBrace => "}",
        RightBracket => "]",
        RightParen => ")",
        Semicolon => ";",
        Slash => "/",
        DoubleSlash => "//",
        Star => "*",
        Tilde => "~",
        TildeEqual => "~=",
    }
}
