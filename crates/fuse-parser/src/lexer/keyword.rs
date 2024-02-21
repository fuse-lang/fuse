use super::{Lexer, LexerResult, Token, TokenKind};
use fuse_common::Span;

impl<'a> Lexer<'a> {
    pub(super) fn keyword(&mut self, start: u32, first: char) -> Option<Token> {
        None
    }
}
