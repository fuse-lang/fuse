use fuse_ast::{Block, Statement};

use crate::{lexer::TokenKind, parsers::statements, Parser, ParserResult};

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

        let start_span = self.start_span();

        match cur_kind {
            kind if kind.is_trivial() => ParserResult::NotFound,
            TokenKind::Semicolon => ParserResult::Ok(self.parse_empty_statement()),
            TokenKind::Const | TokenKind::Let | TokenKind::Global => self
                .parse_variable_declaration(start_span)
                .map(|decl| Statement::VariableDeclaration(decl)),
            _ => todo!(),
        }
    }

    pub(crate) fn parse_empty_statement(&mut self) -> Statement {
        // advance the semicolon token
        self.consume();
        self.ast.empty_statement()
    }
}
