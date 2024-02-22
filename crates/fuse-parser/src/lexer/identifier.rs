use super::{Lexer, Token, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn identifier(&mut self, start: u32, first: char) -> Option<Token> {
        // self.source.advance();
        if !matches!(first, 'a'..='z' | 'A'..='Z' | '_') {
            return None;
        }
        loop {
            let Some(next) = self.source.peek_char() else {
                break;
            };
            if !next.is_ascii_alphabetic() && !next.is_ascii_digit() {
                break;
            }
            self.source.advance();
        }
        Some(self.create(start, TokenKind::Identifier))
    }
}
