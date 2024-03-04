use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{
    BindingPattern, BindingPatternKind, BooleanLiteral, Expression, Function, FunctionParameter,
    FunctionParameters, Identifier,
};
use fuse_common::Span;

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self) -> ParserResult<Expression> {
        match self.cur_kind() {
            TokenKind::True => Ok(Expression::BooleanLiteral(BooleanLiteral {
                span: self.consume().span(),
                value: true,
            })),
            TokenKind::False => Ok(Expression::BooleanLiteral(BooleanLiteral {
                span: self.consume().span(),
                value: false,
            })),
            TokenKind::NumberLiteral => Ok(Expression::NumberLiteral(self.parse_number_literal()?)),
            TokenKind::StringLiteral | TokenKind::InterpolatedStringHead => {
                Ok(Expression::StringLiteral(self.parse_string_literal()?))
            }
            TokenKind::Identifier => self.parse_identifier().map(|id| Expression::Identifier(id)),
            TokenKind::Function | TokenKind::Fn => self
                .parse_function_expression()
                .map(|func| Expression::Function(func)),
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
        let token = self.consume();
        println!("{token:?}, {:?}", self.cur_kind());
        let params = self.parse_function_parameters()?;
        Ok(Function {
            span: self.end_span(start),
            params,
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
}
