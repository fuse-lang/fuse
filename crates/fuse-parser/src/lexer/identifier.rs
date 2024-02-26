use super::{Lexer, Token, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn identifier(&mut self, start: u32, first: char) -> Option<Token> {
        if !match_identifier_start(first) {
            return None;
        }

        let is_raw = matches!(
            (first, self.source.peek_pair()),
            ('r', Some(('#', p))) if match_identifier_start(p)
        );

        // Eat the visited hash.
        if is_raw {
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

#[inline]
fn match_identifier_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}
