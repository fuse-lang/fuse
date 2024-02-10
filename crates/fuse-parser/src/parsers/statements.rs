use fuse_ast::Statement;

use crate::{lexer::TokenKind, Parser, ParserResult};

impl<'a> Parser<'a> {
    pub(crate) fn parse_statement(&mut self) -> ParserResult<Statement> {
        // match self.cur_kind() {
        //     TokenKind::Const => {
        //         let const_token = self.consume().unwrap();
        //         let next_token = match self.cur_token() {
        //             Some(token) => token,
        //             None => return ParserResult::Err,
        //         };
        //
        //         // match next_token.kind() {
        //         //     TokenKind::Identifier => ParserResult<self.ast.const_assignment(match)
        //         // }
        //     }
        //     _ => {}
        // }
        let semicolon = self.consume_if(TokenKind::Semicolon);
        let statement = match semicolon {
            ParserResult::Ok(_) => todo!("self.ast.statement_with_semicolon(semicolon)"),
            _ => self.ast.statement(),
        };
        ParserResult::Ok(statement);
        todo!()
    }
}
