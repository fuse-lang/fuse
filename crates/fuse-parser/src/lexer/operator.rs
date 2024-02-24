use super::{Lexer, Token, TokenKind};
use crate::flash_match;

impl<'a> Lexer<'a> {
    pub(super) fn operator(&mut self, start: u32, peek: char) -> Option<Token> {
        let source = &mut self.source;
        let kind = flash_match! ((source, start, peek) {
            '=' => {
                "" => TokenKind::Eq,
            }
            ';' => {
                "" => TokenKind::Semicolon,
            }
            '-' => {
                "" => TokenKind::Minus,
            }
            '+' => {
                "" => TokenKind::Plus,
            }
        })?;

        Some(self.create(start, kind))
    }
}
