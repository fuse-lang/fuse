use std::rc::Rc;

use crate::{
    lexer::{StringData, StringValue},
    Parser, ParserResult,
};
use fuse_ast::{Atom, StringLiteral, StringLiteralSegment, StringSegment};

impl<'a> Parser<'a> {
    pub(crate) fn parse_string_literal(&mut self) -> ParserResult<StringLiteral> {
        let token = self.consume();

        let view = self.view_token(*token);

        let str_data = self.lexer.eat_string_data(&*token);
        let literal = match str_data.value {
            StringValue::Escaped(val) => StringLiteralSegment::Escaped(Atom(Rc::from(val))),
            StringValue::Unescaped(span) => StringLiteralSegment::Unescaped(span),
        };

        Ok(StringLiteral {
            span: token.span,
            segments: vec![StringSegment::Literal(literal)],
        })
    }
}
