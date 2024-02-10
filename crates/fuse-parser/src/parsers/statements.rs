use fuse_ast::{Block, Statement};

use crate::{lexer::TokenKind, Parser, ParserResult};

impl<'a> Parser<'a> {
    pub(crate) fn parse_block(&mut self) -> ParserResult<Block> {
        self.parse_statements().map(|stmts| self.ast.block(stmts))
    }

    pub(crate) fn parse_statements(&mut self) -> ParserResult<Vec<Statement>> {
        let mut statements = Vec::new();

        while !self.at(TokenKind::Eof) {
            match self.parse_statement() {
                ParserResult::Ok(stmt) => {
                    statements.push(stmt);
                }
                ParserResult::Err => {
                    if statements.is_empty() {
                        return ParserResult::Err;
                    } else {
                        break;
                    }
                }
                ParserResult::NotFound => break,
            }
        }

        ParserResult::Ok(statements)
    }

    pub(crate) fn parse_statement(&mut self) -> ParserResult<Statement> {
        let Ok(cur_kind) = self.cur_kind() else {
            return ParserResult::Err;
        };
        let semicolon = self.consume_if(TokenKind::Semicolon);
        let statement = match semicolon {
            ParserResult::Ok(_) => todo!("self.ast.statement_with_semicolon(semicolon)"),
            _ => self.ast.empty_statement(),
        };
        ParserResult::Ok(statement);

        match cur_kind {
            kind if kind.is_trivial() => ParserResult::NotFound,
            TokenKind::Const => {
                let const_token = self.consume().unwrap();
                let next_token = match self.cur_token() {
                    Ok(token) => token,
                    Err(_) => return ParserResult::Err,
                };

                match next_token.kind() {
                    TokenKind::Identifier => ParserResult::Err,
                    _ => ParserResult::Err,
                }
            }
            _ => todo!(),
        }
    }
}
