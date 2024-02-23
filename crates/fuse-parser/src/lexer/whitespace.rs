use super::{Lexer, Token, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn whitespace(&mut self, start: u32, peek: char) -> Option<Token> {
        if !self.is_whitespace(peek) {
            return None;
        }
        self.source.advance();

        while let Some(next) = self.source.peek_char() {
            if self.is_whitespace(next) {
                self.source.advance();
            } else {
                break;
            }
        }

        Some(self.create(start, TokenKind::Whitespace))
    }

    #[inline]
    fn is_whitespace(&self, c: char) -> bool {
        matches!(c, ' ' | '\t' | '\n' | '\r')
    }
}
