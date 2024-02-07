use fuse_ast::{ast_builder, Block, Statement};

use crate::{lexer::Symbol, Parser, ParserResult};

impl Parser {
    pub(crate) fn parse_chunk(&mut self) -> ParserResult<Block> {
        ParserResult::NotFound
    }

    pub(crate) fn parse_block(&mut self) -> ParserResult<Block> {
        let mut statements = Vec::new();

        loop {
            match parse_stmt(self) {
                ParserResult::Ok(stmt) => {
                    let semicolon = self.consume_if(Symbol::Semicolon);
                    statements.push((stmt, semicolon));
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
}

fn parse_stmt(parser: &mut Parser) -> ParserResult<Statement> {
    ParserResult::NotFound
}
