use super::{Lexer, Token, TokenKind};
use crate::flash_match;

impl<'a> Lexer<'a> {
    pub(super) fn operator(&mut self, start: u32, first: char) -> Option<Token> {
        let source = &mut self.source;
        let kind = flash_match! ((source, start, first) {
            '=' => {
                "" => TokenKind::Eq,
            }
        })?;

        Some(self.create(start, kind))
    }
}
