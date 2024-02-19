use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{BindingPattern, BindingPatternKind};

impl<'a> Parser<'a> {
    pub(crate) fn parse_binding(&mut self) -> ParserResult<BindingPattern> {
        match self.cur_kind().unwrap() {
            TokenKind::LParen => self.parse_tuple_binding(),
            _ => self.parse_identifier_binding(),
        }
    }

    fn parse_tuple_binding(&mut self) -> ParserResult<BindingPattern> {
        self.consume();
        if self.nth_kind(1).unwrap() == TokenKind::RParen {
            let binding = self.parse_identifier_binding();
            self.consume();
            return binding;
        }
        ParserResult::Ok(BindingPattern {
            kind: BindingPatternKind::IdentifierBinding,
            type_annotation: None,
            optional: false,
        })
    }

    fn parse_identifier_binding(&mut self) -> ParserResult<BindingPattern> {
        ParserResult::Err
    }
}
