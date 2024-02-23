use super::{Lexer, Token, TokenKind};
use crate::flash_match;

impl<'a> Lexer<'a> {
    pub(super) fn number(&mut self, start: u32, first: char) -> Option<Token> {
        if !first.is_ascii_digit() {
            return None;
        }
        self.source.advance();

        while let Some(next) = self.source.peek_char() {
            if next.is_ascii_digit() {
                self.source.advance();
            } else {
                break;
            }
        }

        Some(self.create(start, TokenKind::NumberLiteral))
    }
}
