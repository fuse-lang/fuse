use fuse_ast::{Block, Statement};
use fuse_common::Span;

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
                ParserResult::Err(error) => {
                    return ParserResult::Err(error);
                }
            }
        }

        ParserResult::Ok(statements)
    }

    pub(crate) fn parse_statement(&mut self) -> ParserResult<Statement> {
        let cur_kind = self.cur_kind();

        match cur_kind {
            TokenKind::Semicolon => ParserResult::Ok(self.parse_empty_statement()),

            TokenKind::Const | TokenKind::Let | TokenKind::Global => self
                .parse_variable_declaration()
                .map(|decl| Statement::VariableDeclaration(decl)),

            TokenKind::True
            | TokenKind::False
            | TokenKind::NumberLiteral
            | TokenKind::StringLiteral
            | TokenKind::InterpolatedStringHead
            | TokenKind::Identifier
            | TokenKind::Function
            | TokenKind::Fn => self
                .parse_expression()
                .map(|expr| Statement::Expression(expr)),

            kind if kind.is_trivial() => {
                unreachable!("All trivial tokens should be eaten by a `TokenReference`.")
            }
            _ => Err(Self::unexpected_error(self.cur_token())),
        }
    }

    pub(crate) fn parse_empty_statement(&mut self) -> Statement {
        let start = self.start_span();
        // advance the semicolon token
        self.consume();
        let span = self.end_span(start);
        self.ast.empty_statement(span)
    }
}
