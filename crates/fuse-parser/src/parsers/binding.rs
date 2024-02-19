use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::BindingPattern;

impl<'a> Parser<'a> {
    pub(crate) fn parse_binding(&mut self) -> ParserResult<BindingPattern> {
        ParserResult::Ok(BindingPattern {
            kind: BindingPatternKind::IdentifierBinding,
            type_annotation: None,
            optional: false,
        })
    }
}
