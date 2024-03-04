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
                "lseif" => TokenKind::ElseIf,
                "xport" => TokenKind::Export,
                "lse" => TokenKind::Else,
                "num" => TokenKind::Enum,
                "nd" => TokenKind::End,
            }
            'f' => {
                "unction" => TokenKind::Function,
                "alse" => TokenKind::False,
                "rom" => TokenKind::From,
                "or" => TokenKind::For,
                "n" => TokenKind::Fn,
            }
            'g' => {
                "lobal" => TokenKind::Global,
            }
            'i' => {
                "mport" => TokenKind::Import,
                "mpl" => TokenKind::Impl,
                "f" => TokenKind::If,
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
                "tatic" => TokenKind::Static,
                "truct" => TokenKind::Struct,
                "elf" => TokenKind::LowSelf,
            }
            'S' => {
                "elf" => TokenKind::CapSelf,
            }
            't' => {
                "rait" => TokenKind::Trait,
                "hen" => TokenKind::Then,
                "rue" => TokenKind::True,
                "ype" => TokenKind::Type,
            }
            'u' => {
                "nknown" => TokenKind::Unknown,
                "nsafe" => TokenKind::Unsafe,
                "nion" => TokenKind::Union,
                "ntil" => TokenKind::Until,
            }
            'w' => {
                "hile" => TokenKind::While,
                "hen" => TokenKind::When,
            }
        })?;
        Some(self.create(start, kind))
    }
}
