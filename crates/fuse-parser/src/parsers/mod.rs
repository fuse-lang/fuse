mod binding;
mod declarations;
mod expressions;
mod numbers;
mod statements;
mod strings;

use fuse_ast::Chunk;
use crate::{Parser, ParserResult};

impl<'a> Parser<'a> {
    pub(crate) fn parse_chunk(&mut self) -> ParserResult<Chunk> {
        let span = fuse_common::Span::new(0, self.source.len() as u32);
        let body = self.parse_block()?;

        let chunk = self.ast.chunk(span, body);
        ParserResult::Ok(chunk)
    }
}
