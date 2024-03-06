use super::{Lexer, Token, TokenKind};
use crate::flash_match;

impl<'a> Lexer<'a> {
    pub(super) fn operator(&mut self, start: u32, first: char) -> Option<Token> {
        let source = &mut self.source;
        let kind = flash_match! ((source, start, first) {
            ',' => {
                "" => TokenKind::Comma,
            }
            '.' => {
                ".." => TokenKind::Dot3,
                "." => TokenKind::Dot2,
                "" => TokenKind::Dot,
            }
            '=' => {
                ">" => TokenKind::Arrow,
                "" => TokenKind::Eq,
            }
            ';' => {
                "" => TokenKind::Semicolon,
            }
            '+' => {
                "" => TokenKind::Plus,
            }
            '-' => {
                ">" => TokenKind::ThinArrow,
                "" => TokenKind::Minus,
            }
            '*' => {
                "" => TokenKind::Star,
            }
            '/' => {
                "/" => TokenKind::Slash2,
                "" => TokenKind::Slash,
            }
            '(' => {
                "" => TokenKind::LParen,
            }
            ')' => {
                "" => TokenKind::RParen,
            }
            '{' => {
                "" => TokenKind::LCurly,
            }
            '}' => {
                "" => TokenKind::RCurly,
            }
            '<' => {
                "=" => TokenKind::LtEq,
                "" => TokenKind::LAngle,
            }
            '>' => {
                "=" => TokenKind::GtEq,
                "" => TokenKind::RAngle,
            }
        })?;

        Some(self.create(start, kind))
    }
}
