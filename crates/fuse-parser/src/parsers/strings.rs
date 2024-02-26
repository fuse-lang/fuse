use std::rc::Rc;

use crate::{Parser, ParserResult};
use fuse_ast::{Atom, StringLiteral, StringLiteralSegment, StringSegment};

impl<'a> Parser<'a> {
    pub(crate) fn parse_string_literal(&mut self) -> ParserResult<StringLiteral> {
        let token = self.consume();

        let view = self.view_token(*token);

        let str_data = self.lexer.get_string_data(&*token);

        Ok(StringLiteral {
            span: token.span,
            segments: vec![StringSegment::Literal(StringLiteralSegment::Escaped(Atom(
                Rc::from(str_data.data.clone()),
            )))],
        })
    }
}
