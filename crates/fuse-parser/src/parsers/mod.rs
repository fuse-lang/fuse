mod statements;

use fuse_ast::{Block, Chunk};

use crate::{lexer::TokenKind, Parser, ParserResult};

impl<'a> Parser<'a> {
    pub(crate) fn parse_chunk(&mut self) -> ParserResult<Chunk> {
        let span = fuse_common::Span::new(0, self.source.len() as u32);
        let body = match self.parse_block() {
            ParserResult::Ok(block) => block,
            _ => self.ast.block(),
        };

        let chunk = self.ast.chunk(span, body);
        ParserResult::Ok(chunk)
    }

    pub(crate) fn parse_block(&mut self) -> ParserResult<Block> {
        let mut statements = Vec::new();

        while !self.at(TokenKind::Eof) {
            match self.parse_statement() {
                ParserResult::Ok(stmt) => {
                    statements.push(stmt);
                }
                ParserResult::NotFound => break,
                ParserResult::Err => {
                    if statements.is_empty() {
                        return ParserResult::Err;
                    } else {
                        break;
                    }
                }
            }
        }

        ParserResult::Ok(self.ast.block_with_statements(statements))
    }
}
