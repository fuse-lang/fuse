use crate::{lexer::TokenKind, Parser, ParserResult};
use fuse_ast::{
    EnumDeclaration, EnumVariant, Function, VariableDeclaration, VariableDeclarationKind,
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_variable_declaration(&mut self) -> ParserResult<VariableDeclaration> {
        let decl_kind = match self.cur_kind() {
            TokenKind::Let => VariableDeclarationKind::Let,
            TokenKind::Const => VariableDeclarationKind::Const,
            TokenKind::Global => VariableDeclarationKind::Global,
            TokenKind::Local => {
                self.push_error(Self::unexpected_error(self.cur_token()));
                VariableDeclarationKind::Let
            }
            _ => return Err(Self::unexpected_error(self.cur_token())),
        };

        let start = self.start_span();

        self.consume();

        let binding = self.parse_binding()?;

        let expression = if let Some(_) = self.consume_if(TokenKind::Eq) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(self
            .ast
            .variable_declaration(self.end_span(start), decl_kind, binding, expression))
    }

    pub(crate) fn parse_function_declaration(&mut self) -> ParserResult<Function> {
        self.parse_function(true)
    }

    pub(crate) fn parse_enum_declaration(&mut self) -> ParserResult<EnumDeclaration> {
        debug_assert!(self.at(TokenKind::Enum));
        let start = self.start_span();
        // Consume the enum keyword.
        self.consume();

        let identifier = self.parse_identifier()?;
        let mut variants: Vec<EnumVariant> = Vec::new();
        while !self.at(TokenKind::End) {
            let identifier = self.parse_identifier()?;
            let value = if self.consume_if(TokenKind::Eq).is_some() {
                Some(self.parse_expression()?)
            } else {
                None
            };
            variants.push(EnumVariant { identifier, value })
        }
        // consume the end token
        self.consume();
        Ok(EnumDeclaration {
            span: self.end_span(start),
            identifier,
            variants,
        })
    }
}
