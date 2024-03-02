use fuse_common::Span;

use super::{Lexer, LexerContext, StringData, StringValue, Token, TokenKind, TokenReference};

impl<'a> Lexer<'a> {
    pub(crate) fn promote_to_interpolated_string(
        &mut self,
        start: u32,
        has_escaped: bool,
        value: Vec<char>,
    ) -> Token {
        self.context.push(LexerContext::Interpolation);
        let token = self.create(start, TokenKind::InterpolatedStringHead);
        let value = if has_escaped {
            // TODO: look into more efficent ways to collect char array into string.
            StringValue::Escaped(value.iter().collect())
        } else {
            StringValue::Unescaped(Span::new(start, 1))
        };
        self.set_string_data(
            token,
            StringData {
                quote: '\'',
                value,
                terminated: true,
                unicode: true,
                raw: true,
            },
        );
        token
    }

    pub fn next_string_interpolation_segment(&self, last_token: TokenReference) {
        // match substitution segment.
        // after each middle segment we expect an expression.
    }
}
