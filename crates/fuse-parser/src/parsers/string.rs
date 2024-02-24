use crate::{Parser, ParserResult};
use fuse_ast::StringLiteral;

impl<'a> Parser<'a> {
    pub(crate) fn parse_string_literal(&mut self) -> ParserResult<StringLiteral> {
        todo!()
    }
}
