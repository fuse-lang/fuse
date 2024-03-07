use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{BindingIdentifier, BindingPattern, BindingPatternKind};

impl<'a> Parser<'a> {
    pub(crate) fn parse_binding(&mut self) -> ParserResult<BindingPattern> {
        match self.cur_kind() {
            TokenKind::LParen => self.parse_tuple_binding_pattern(),
            _ => self.parse_binding_identifier_pattern(),
        }
    }

    fn parse_tuple_binding_pattern(&mut self) -> ParserResult<BindingPattern> {
        self.consume();

        // fallback to identifier binding if it is just a variable in parentheses
        if self.nth_kind(1) == TokenKind::RParen {
            let binding = self.parse_binding_identifier_pattern();
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

    fn parse_binding_identifier_pattern(&mut self) -> ParserResult<BindingPattern> {
        if !self.cur_kind().is_valid_identifier() && !self.at(TokenKind::Mut) {
            return Err(Self::unexpected_error(self.cur_token()));
        }

        let identifier = self.parse_binding_identifier();
        Ok(self.ast.binding_identifier_pattern(identifier, None, false))
    }

    pub(crate) fn parse_binding_identifier(&mut self) -> BindingIdentifier {
        let mut span = self.start_span();
        let mutable = self.consume_if(TokenKind::Mut).is_some();
        let token = self.consume();
        let name = self.view_token(*token);

        span = self.end_span(span);

        let atom = self.ast.atom(name);

        self.ast.binding_identifier(span, atom, mutable)
    }
}
