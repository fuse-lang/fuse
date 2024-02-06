use fuse_ast::{ast_builder, Block};

use crate::{Parser, ParserResult, Symbol};

impl Parser {
    pub(crate) fn parse_block(&mut self) -> ParserResult<Block> {
        let mut stmts = Vec::new();

        loop {
            match parse_stmt(self) {
                ParserResult::Ok(stmt) => {
                    let semicolon = self.consume_if(Symbol::Semicolon);
                    stmts.push((stmt, semicolon));
                }
                ParserResult::Panic(_) => break,
                ParserResult::Err(err) => {
                    if stmts.is_empty() {
                        return ParserResult::Err(err);
                    } else {
                        break;
                    }
                }
            }
        }

        ParserResult::Ok(Block { stmts })
    }
}

fn parse_stmt(parser: &mut Parser) -> ParserResult<ast::Statement> {
    ParserResult::Fatal
}
