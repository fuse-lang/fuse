use std::rc::Rc;

use crate::{
    lexer::{StringData, StringValue, Token, TokenKind, TokenReference},
    Parser, ParserResult,
};
use fuse_ast::{
    Atom, InterpolatedStringSegment, StringLiteral, StringLiteralSegment, StringSegment,
};
use fuse_common::Span;

impl<'a> Parser<'a> {
    pub(crate) fn parse_string_literal(&mut self) -> ParserResult<StringLiteral> {
        if self.at(TokenKind::InterpolatedStringHead) {
            return self.parse_string_interpolation();
        }

        let token = self.consume();

        let str_data = self.lexer.eat_string_data(&*token);
        let literal = match str_data.value {
            StringValue::Escaped(val) => StringLiteralSegment::Escaped(Atom(Rc::from(val))),
            StringValue::Unescaped(span) => StringLiteralSegment::Unescaped(span),
        };

        Ok(StringLiteral {
            span: token.span,
            segments: vec![StringSegment::Literal(literal)],
        })
    }

    fn parse_string_interpolation(&mut self) -> ParserResult<StringLiteral> {
        let mut segments: Vec<StringSegment> = Vec::new();
        let head = self.consume();
        let head_data = self.lexer.eat_string_data(&*head);
        let tail = loop {
            let expression = self.parse_expression()?;
            self.lexer.follow_string_interpolation(&head_data);
            let next_segment = self.consume();
            segments.push(StringSegment::Interpolated(InterpolatedStringSegment {
                expression,
                format: fuse_ast::InterpolationFormat::Display,
            }));
            if next_segment.kind() == TokenKind::InterpolatedStringTail {
                break self.consume();
            }
        };

        Ok(StringLiteral {
            span: Span::new(head.start(), tail.end()),
            segments,
        })
    }
}
