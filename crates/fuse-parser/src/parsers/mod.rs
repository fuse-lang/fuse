use fuse_ast::{Block, Chunk, Statement};

use crate::{lexer::TokenKind, Parser, ParserResult};

impl<'a> Parser<'a> {
    pub(crate) fn parse_chunk(&mut self) -> ParserResult<Chunk> {
        let span = fuse_common::Span::new(0, self.source.len() as u32);
        let body = match self.parse_block() {
            ParserResult::Ok(block) => block,
            _ => self.factory.block(),
        };

        let chunk = self.factory.chunk(span, body);
        ParserResult::Ok(chunk)
    }

    pub(crate) fn parse_block(&mut self) -> ParserResult<Block> {
        let mut statements = Vec::new();

        loop {
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

        ParserResult::Ok(self.factory.block_with_statements(statements))
    }

    fn parse_statement(&mut self) -> ParserResult<Statement> {
        let Some(current) = self.cur_token() else {
            return ParserResult::NotFound;
        };

        match current.kind() {
            TokenKind::Const => {
                let const_token = self.consume().unwrap();
                let next_token = match self.cur_token() {
                    Some(token) => token,
                    None => return ParserResult::Err,
                };

                // match next_token.kind() {
                //     TokenKind::Identifier => ParserResult<self.factory.const_assignment(match)
                // }
            }
            _ => {}
        }
        let statement = self.factory.statement();
        let semicolon = self.consume_if(TokenKind::Semicolon);
        ParserResult::Ok(statement)
    }
}
