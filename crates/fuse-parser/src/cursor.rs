use crate::{
    lexer::{LexerResult, TokenKind, TokenReference},
    Parser, ParserResult,
};

impl<'a> Parser<'a> {
    pub(crate) fn cur_token(&self) -> Option<&TokenReference> {
        match self.lexer.current() {
            LexerResult::Ok(token) | LexerResult::Recovered(token, _) => Some(token),
            _ => None,
        }
    }

    pub(crate) fn cur_kind(&self) -> Option<TokenKind> {
        self.cur_token().map(|token| token.kind)
    }

    pub(crate) fn consume(&mut self) -> ParserResult<TokenReference> {
        let token = self.lexer.consume();

        match token {
            LexerResult::Ok(token) => ParserResult::Ok(token),
            LexerResult::Recovered(token, errors) => {
                self.push_errors(errors);
                ParserResult::Ok(token)
            }
            LexerResult::Fatal(errors) => {
                self.push_errors(errors);
                ParserResult::Err
            }
        }
    }

    pub fn consume_if(&mut self, kind: TokenKind) -> ParserResult<TokenReference> {
        if self.at(kind) {
            self.consume()
        } else {
            ParserResult::NotFound
        }
    }

    pub fn at(&self, kind: TokenKind) -> bool {
        matches!(self.cur_token(), Some(token) if token.kind == kind)
    }
}
