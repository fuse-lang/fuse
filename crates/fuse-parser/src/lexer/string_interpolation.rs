use fuse_common::Span;

use super::{Lexer, LexerContext, StringData, StringValue, Token, TokenKind, TokenReference};

impl<'a> Lexer<'a> {
    pub(crate) fn promote_to_interpolated_string(&mut self, start: u32, data: StringData) -> Token {
        self.context.push(LexerContext::Interpolation);
        let token = self.create(start, TokenKind::InterpolatedStringHead);

        self.set_string_data(token, data);
        token
    }

    pub fn follow_string_interpolation(&mut self) -> Token {
        if self.source.next_char().unwrap() != '}' {
            panic!("Unterminated string interpolation!");
        }
        todo!()
        // match substitution segment.
        // after each middle segment we expect an expression.
    }
}
