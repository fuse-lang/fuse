use crate::{
    lexer::{TokenKind, TokenReference},
    Parser, ParserResult,
};
use fuse_ast::{
    BindingPattern, BindingPatternKind, Function, FunctionBody, FunctionParameter,
    FunctionParameters, TypeAnnotation,
};
use fuse_common::Span;
impl<'a> Parser<'a> {
    pub(crate) fn parse_function(&mut self, expect_identifier: bool) -> ParserResult<Function> {
        let start = self.start_span();
        // Consume the keyword
        self.consume();
        let identifier = if expect_identifier {
            Some(self.parse_identifier()?)
        } else {
            None
        };
        let params = self.parse_function_parameters()?;
        let return_type = self.parse_function_return_type()?;
        let body = self.parse_function_body()?;
        Ok(Function {
            span: self.end_span(start),
            identifier,
            params,
            return_type,
            body,
        })
    }
    pub(crate) fn parse_function_parameters(&mut self) -> ParserResult<FunctionParameters> {
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
        let mut seen_comma = true;

        while self.at(TokenKind::Identifier) {
            if !seen_comma {
                return Err(Self::unexpect_token_kind_error(
                    self.cur_token(),
                    TokenKind::Comma,
                ));
            }

            let (param, comma) = self.parse_function_parameter()?;
            seen_comma = comma.is_some();
            params.push(param);
        }

        let close = self.consume_expect(TokenKind::RParen)?;
        Ok(FunctionParameters {
            span: Span::new(open.start(), close.end()),
            items: params,
            rest: None,
        })
    }

    fn parse_function_parameter(
        &mut self,
    ) -> ParserResult<(FunctionParameter, Option<TokenReference>)> {
        let binding = self.parse_binding()?;
        let BindingPattern {
            kind: BindingPatternKind::Identifier(kind),
            ..
        } = &binding
        else {
            todo!(
                "Diagnosis error here,\
                            we don't allow variable deconstruction for function parameters."
            );
        };

        Ok((
            FunctionParameter {
                span: kind.span,
                pattern: binding,
            },
            self.consume_if(TokenKind::Comma),
        ))
    }

    pub(crate) fn parse_function_return_type(&mut self) -> ParserResult<Option<TypeAnnotation>> {
        if self.consume_if(TokenKind::ThinArrow) == None {
            return Ok(None);
        }
        self.parse_type_annotation().map(|t| Some(t))
    }

    pub(crate) fn parse_function_body(&mut self) -> ParserResult<FunctionBody> {
        if let Some(_) = self.consume_if(TokenKind::Arrow) {
            Ok(FunctionBody::Expression(self.parse_expression()?))
        } else {
            Ok(FunctionBody::Block(self.parse_block()?))
        }
    }
}
