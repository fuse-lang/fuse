use super::{Lexer, LexerContext, Token, TokenKind, TokenReference};

impl<'a> Lexer<'a> {
    pub(crate) fn promote_to_interpolated_string(&mut self, start: u32) -> Token {
        self.context.push(LexerContext::Interpolation);
        self.create(start, TokenKind::InterpolatedStringHead)
    }

    pub fn next_string_interpolation_segment(&self, last_token: TokenReference) {
        // match substitution segment.
        // after each middle segment we expect an expression.
    }
}
