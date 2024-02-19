use crate::{
    lexer::{LexerResult, TokenKind, TokenReference},
    Parser, ParserResult,
};

type Result<R> = std::result::Result<R, ()>;

impl<R> Into<ParserResult<R>> for Result<R> {
    fn into(self) -> ParserResult<R> {
        match self {
            Ok(value) => ParserResult::Ok(value),
            Err(_) => ParserResult::Err,
        }
    }
}

impl<'a> Parser<'a> {
    pub(crate) fn cur_token(&self) -> Result<&TokenReference> {
        match self.lexer.current() {
            LexerResult::Ok(token) | LexerResult::Recovered(token, _) => Ok(token),
            LexerResult::Fatal(_) => Err(()),
        }
    }

    pub(crate) fn cur_kind(&self) -> Result<TokenKind> {
        self.cur_token().map(|token| token.kind)
    }

    pub(crate) fn consume(&mut self) -> Result<TokenReference> {
        let token = self.lexer.consume();

        match token {
            LexerResult::Ok(token) => {
                self.prev_token_end = token.end();
                Ok(token)
            }
            LexerResult::Recovered(token, errors) => {
                self.push_errors(errors);
                Ok(token)
            }
            LexerResult::Fatal(errors) => {
                self.push_errors(errors);
                Err(())
            }
        }
    }

    pub fn consume_if(&mut self, kind: TokenKind) -> ParserResult<TokenReference> {
        if self.at(kind) {
            self.consume().into()
        } else {
            ParserResult::NotFound
        }
    }

    pub fn at(&self, kind: TokenKind) -> bool {
        matches!(self.cur_token(), Ok(token) if token.kind == kind)
    }
}
