use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{
    BindingPattern, BindingPatternKind, Block, BooleanLiteral, Expression, Function,
    FunctionParameter, FunctionParameters, Identifier, TypeAnnotation,
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
            TokenKind::Function | TokenKind::Fn => self
                .parse_function_expression()
                .map(|func| self.ast.function_expression(func)),
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

    pub(crate) fn parse_function_expression(&mut self) -> ParserResult<Function> {
        let start = self.start_span();
        // Consume the keyword
        self.consume();
        let params = self.parse_function_parameters()?;
        let return_type = self.parse_function_return_type()?;
        let end = self.consume_expect(TokenKind::End);
        Ok(Function {
            span: self.end_span(start),
            params,
            return_type,
            body: None,
        })
    }

    fn parse_function_parameters(&mut self) -> ParserResult<FunctionParameters> {
        let open = self.consume_expect(TokenKind::LParen)?;
        // Empty function parameters
        if let Some(close) = self.consume_if(TokenKind::RParen) {
            return Ok(FunctionParameters {
                span: Span::new(open.start(), close.end()),
                items: Vec::new(),
                rest: None,
            });
        }

        let mut params = Vec::new();
        let mut first_param = true;
        while self.at(TokenKind::Identifier) {
            if first_param {
                first_param = false;
            } else {
                self.consume_expect(TokenKind::Comma)?;
            }

            let binding = self.parse_binding()?;
            match &binding {
                BindingPattern {
                    kind: BindingPatternKind::Identifier(kind),
                    ..
                } => params.push(FunctionParameter {
                    span: kind.span,
                    pattern: binding,
                }),
                _ => todo!(
                    "Diagnosis error here,\
                            we don't allow variable deconstruction for function parameters."
                ),
            }
        }
        // accept trailing commas.
        self.consume_if(TokenKind::Comma);

        let close = self.consume_expect(TokenKind::RParen)?;
        Ok(FunctionParameters {
            span: Span::new(open.start(), close.end()),
            items: params,
            rest: None,
        })
    }

    fn parse_function_return_type(&mut self) -> ParserResult<Option<TypeAnnotation>> {
        if self.consume_if(TokenKind::ThinArrow) == None {
            return Ok(None);
        }
        self.parse_type_annotation().map(|t| Some(t))
    }

    fn parse_function_body(&mut self) -> ParserResult<Block> {
        if let Some(_) = self.consume_if(TokenKind::Arrow) {
            self.parse_expression();
            todo!()
        } else {
            self.parse_block()
        }
    }
}
