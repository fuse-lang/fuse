use fuse_ast::{Block, Statement};

use crate::{lexer::TokenKind, Parser, ParserResult};

impl<'a> Parser<'a> {
    /// Parse a block of statements until and including the `end` token.
    pub(crate) fn parse_block(&mut self) -> ParserResult<Block> {
        let result = self
            .parse_statements(|kind| kind != TokenKind::End)
            .map(|stmts| self.ast.block(stmts));
        // Eat the end token.
        self.consume();
        result
    }

    /// Parse a block of statements while the predicate returns `true`.
    pub(crate) fn parse_block_while<F: Fn(TokenKind) -> bool>(
        &mut self,
        predicate: F,
    ) -> ParserResult<Block> {
        self.parse_statements(predicate)
            .map(|stmts| self.ast.block(stmts))
    }

    pub(crate) fn parse_statements<F: Fn(TokenKind) -> bool>(
        &mut self,
        predicate: F,
    ) -> ParserResult<Vec<Statement>> {
        let mut statements = Vec::new();

        while !self.at(TokenKind::Eof) && predicate(self.cur_kind()) {
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
                .map(|decl| self.ast.variable_declaration_statement(decl)),

            TokenKind::True
            | TokenKind::False
            | TokenKind::NumberLiteral
            | TokenKind::StringLiteral
            | TokenKind::InterpolatedStringHead
            | TokenKind::Identifier
            | TokenKind::If => self
                .parse_expression()
                .map(|expr| self.ast.expression_statement(expr)),

            // Short circuit the unary operators to prevent extra checks.
            TokenKind::Not | TokenKind::Plus | TokenKind::Minus => self
                .parse_unary_operator_expression()
                .map(|expr| self.ast.expression_statement(expr)),

            TokenKind::Function | TokenKind::Fn => {
                if self.nth_kind(1) == TokenKind::Identifier {
                    // function declaration
                    self.parse_function_declaration()
                        .map(|func| self.ast.function_declaration_statement(func))
                } else {
                    // function expression
                    self.parse_function_expression()
                        .map(|expr| self.ast.expression_statement(expr))
                }
            }

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
