use crate::{lexer::TokenKind, Parser, ParserResult};

use fuse_ast::TypeAnnotation;

impl<'a> Parser<'a> {
    pub(crate) fn parse_type_annotation(&mut self) -> ParserResult<TypeAnnotation> {
        println!("Type annotation not implemented yet!");
        Ok(TypeAnnotation {})
    }
}
