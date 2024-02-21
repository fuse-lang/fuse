use super::{Lexer, Token, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn whitespace(&mut self, start: u32, first: char) -> Option<Token> {
        if !self.is_whitespace(first) {
            return None;
        }

        while let Some(next) = self.source.peek_char() {
            if self.is_whitespace(next) {
                self.source.advance();
            } else {
                break;
            }
        }

        Some(self.create(start, TokenKind::Whitespace))
    }

    fn is_whitespace(&self, c: char) -> bool {
        matches!(c, ' ' | '\t' | '\n' | '\r')
    }
}
