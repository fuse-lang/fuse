use super::{Lexer, LexerResult, Token, TokenKind};
use fuse_common::Span;

impl<'a> Lexer<'a> {
    pub(super) fn whitespace(&mut self, start: u32, first: char) -> Option<LexerResult<Token>> {
        let mut end = start + first.len_utf8() as u32;
        while let Some(next) = self.source.peek_char() {
            if next == ' ' || next == '\t' {
                end += next.len_utf8() as u32;
                self.source
                    .next_char()
                    .expect("we peeked this character before");
            } else if next == '\n' {
                end += next.len_utf8() as u32;
                self.source
                    .next_char()
                    .expect("we peeked this character before");
                break;
            } else if next == '\r' && self.source.peek_char2() == Some('\n') {
                let slash_r = self
                    .source
                    .next_char()
                    .expect("we peeked this character before");
                let slash_n = self
                    .source
                    .next_char()
                    .expect("we peeked this character before");
                end += slash_r.len_utf8() as u32 + slash_n.len_utf8() as u32;
                break;
            } else {
                break;
            }
        }

        Some(LexerResult::Ok(Token::new(
            Span::new(start, end),
            TokenKind::Whitespace,
        )))
    }
}
