use fuse_ast::{ast_builder, Block, Statement};

use crate::{lexer::Symbol, Parser, ParserResult};

impl<'a> Parser<'a> {
    pub(crate) fn parse_chunk(&mut self) -> ParserResult<Block> {
        ParserResult::NotFound
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

        ParserResult::Ok(ast_builder::block(statements))
    }

    fn parse_statement(&mut self) -> ParserResult<Statement> {
        let statement = ast_builder::statement();
        let semicolon = self.consume_if(Symbol::Semicolon);
        ParserResult::Ok(statement)
    }
}
