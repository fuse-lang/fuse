use super::{Lexer, Token, TokenKind};
use crate::flash_match;

impl<'a> Lexer<'a> {
    pub(super) fn keyword(&mut self, start: u32, first: char) -> Option<Token> {
        let source = &mut self.source;
        let kind = flash_match! ((source, start, first) {
            'a' => {
                "nd" => TokenKind::And,
                "s" => TokenKind::As,
            }
            'b' => {
                "reak" => TokenKind::Break,
            }
            'c' => {
                "onst" => TokenKind::Const,
            }
            'd' => {
                "o" => TokenKind::Do,
            }
            'e' => {
                "lse" => TokenKind::Else,
                "lseif" => TokenKind::ElseIf,
                "nd" => TokenKind::End,
                "num" => TokenKind::Enum,
                "xport" => TokenKind::Export,
            }
            'f' => {
                "alse" => TokenKind::False,
                "or" => TokenKind::For,
                "rom" => TokenKind::From,
                "unction" => TokenKind::Function,
                "n" => TokenKind::Fn,
            }
            'g' => {
                "lobal" => TokenKind::Global,
            }
            'i' => {
                "f" => TokenKind::If,
                "mpl" => TokenKind::Impl,
                "mport" => TokenKind::Import,
                "n" => TokenKind::In,
            }
            'l' => {
                "et" => TokenKind::Let,
            }
            'm' => {
                "atch" => TokenKind::Match,
            }
            'n' => {
                "ever" => TokenKind::Never,
                "il" => TokenKind::Nil,
                "ot" => TokenKind::Not,
            }
            'p' => {
                "ub" => TokenKind::Pub,
            }
            'o' => {
                "r" => TokenKind::Or,
            }
            'r' => {
                "epeat" => TokenKind::Repeat,
                "eturn" => TokenKind::Return,
            }
            's' => {
                "elf" => TokenKind::LowSelf,
                "tatic" => TokenKind::Static,
                "truct" => TokenKind::Struct,
            }
            'S' => {
                "elf" => TokenKind::CapSelf,
            }
            't' => {
                "hen" => TokenKind::Then,
                "rait" => TokenKind::Trait,
                "rue" => TokenKind::True,
                "ype" => TokenKind::Type,
            }
            'u' => {
                "nion" => TokenKind::Union,
                "nknown" => TokenKind::Unknown,
                "ntil" => TokenKind::Until,
                "nsafe" => TokenKind::Unsafe,
            }
            'w' => {
                "hen" => TokenKind::When,
                "hile" => TokenKind::While,
            }
        })?;
        Some(self.create(start, kind))
    }
}
