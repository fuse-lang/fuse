use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{BindingPattern, BindingPatternKind};

impl<'a> Parser<'a> {
    pub(crate) fn parse_binding(&mut self) -> ParserResult<BindingPattern> {
        match self.cur_kind() {
            TokenKind::LParen => self.parse_tuple_binding(),
            _ => self.parse_binding_identifier(),
        }
    }

    fn parse_tuple_binding(&mut self) -> ParserResult<BindingPattern> {
        self.consume();

        // fallback to identifier binding if it is just a variable in parentheses
        if self.nth_kind(1) == TokenKind::RParen {
            let binding = self.parse_binding_identifier();
            // consume the closing parentheses
            self.consume();
            return binding;
        }

        ParserResult::Ok(BindingPattern {
            kind: BindingPatternKind::Tuple,
            type_annotation: None,
            optional: false,
        })
    }

    fn parse_binding_identifier(&mut self) -> ParserResult<BindingPattern> {
        if self.cur_kind().is_identifier() {

        }
        todo!()
    }
}
