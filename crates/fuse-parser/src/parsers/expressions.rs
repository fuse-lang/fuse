use crate::{
    lexer::{TokenKind, TokenReference},
    Parser, ParserResult,
};
use fuse_ast::{
    BindingPattern, BindingPatternKind, BooleanLiteral, Expression, Function, FunctionBody,
    FunctionParameter, FunctionParameters, Identifier, If, TypeAnnotation,
};
use fuse_common::Span;

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self) -> ParserResult<Expression> {
        match self.cur_kind() {
            TokenKind::True => {
                let token = self.consume();
                Ok(self.ast.boolean_expression(BooleanLiteral {
                    span: token.span(),
                    value: true,
                }))
            }
            TokenKind::False => {
                let token = self.consume();
                Ok(self.ast.boolean_expression(BooleanLiteral {
                    span: token.span(),
                    value: false,
                }))
            }
            TokenKind::NumberLiteral => {
                let expr = self.parse_number_literal()?;
                Ok(self.ast.number_expression(expr))
            }
            TokenKind::StringLiteral | TokenKind::InterpolatedStringHead => {
                let expr = self.parse_string_literal()?;
                Ok(self.ast.string_expression(expr))
            }
            TokenKind::Identifier => self
                .parse_identifier()
                .map(|id| self.ast.identifier_expression(id)),
            TokenKind::Function | TokenKind::Fn => self.parse_function_expression(),
            TokenKind::If => self.parse_if_expression(),
            _ => Err(Self::unexpected_error(self.cur_token())),
        }
    }

    pub(crate) fn parse_identifier(&mut self) -> ParserResult<Identifier> {
        let token = self.consume();
        let view = self.view_token(*token);
        Ok(Identifier {
            span: token.span(),
            name: self.ast.atom(view),
        })
    }

    pub(crate) fn parse_function_expression(&mut self) -> ParserResult<Expression> {
        self.parse_function(false)
            .map(|func| self.ast.function_expression(func))
    }

    fn parse_if_expression(&mut self) -> ParserResult<Expression> {
        let start = self.start_span();
        // Consume the keyword
        self.consume();
        let cond = self.parse_expression()?;
        self.consume_expect(TokenKind::Then)?;
        let body = self.parse_block_while(|kind| {
            matches! {
                kind,
                    | TokenKind::End
                    | TokenKind::Else
                    | TokenKind::ElseIf
            }
        })?;
        let r#else = match self.consume().kind() {
            TokenKind::End => None,
            _ => todo!(),
        };
        // how to detect end of block?
        // maybe via a predicate function?
        Ok(self.ast.if_expression(If {
            span: self.end_span(start),
            cond,
            body,
            r#else,
        }))
    }
}
