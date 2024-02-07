use crate::{
    lexer::{LexerResult, Symbol, TokenReference},
    Parser, ParserResult,
};

impl Parser {
    pub(crate) fn cur_token(&self) -> Option<&TokenReference> {
        match self.lexer.current() {
            Some(LexerResult::Ok(token) | LexerResult::Recovered(token, _)) => Some(token),
            Some(LexerResult::Fatal(_)) => None,
            None => unreachable!("Attempting to get current token past EOF!"),
        }
    }

    pub(crate) fn consume(&mut self) -> ParserResult<TokenReference> {
        let token = self.lexer.consume();

        match token {
            Some(LexerResult::Ok(token)) => ParserResult::Ok(token),
            Some(LexerResult::Recovered(token, errors)) => {
                for error in errors {
                    self.errors.push(crate::Error::LexerError(error));
                }

                ParserResult::Ok(token)
            }
            Some(LexerResult::Fatal(errors)) => {
                for error in errors {
                    self.errors.push(crate::Error::LexerError(error));
                }

                ParserResult::Err
            }
            None => ParserResult::NotFound,
        }
    }

    pub fn consume_if(&mut self, symbol: Symbol) -> Option<TokenReference> {
        match self.cur_token() {
            Some(token) => {
                if token.is_symbol(symbol) {
                    Some(self.consume().unwrap())
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
