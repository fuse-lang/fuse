use super::{Lexer, Token, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn identifier(&mut self, start: u32, peek: char) -> Option<Token> {
        if !matches!(peek, 'a'..='z' | 'A'..='Z' | '_') {
            return None;
        }

        // Eat the hash symbol if it is a raw identifier.
        if peek == 'r' && self.source.peek_char()? == '#' {
            self.source.advance();
        }

        // Eat the rest of the identifier.
        loop {
            let Some(next) = self.source.peek_char() else {
                break;
            };
            if !next.is_ascii_alphabetic() && !next.is_ascii_digit() && next != '_' {
                break;
            }
            self.source.advance();
        }

        Some(self.create(start, TokenKind::Identifier))
    }
}
