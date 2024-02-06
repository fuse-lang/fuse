mod lexer;
mod parsers;

use fuse_ast::Block;
use lexer::{Lexer, LexerResult, Symbol, TokenReference};

pub enum Error {
    LexerError(lexer::LexerError),
}

pub enum ParserResult<T> {
    Ok(T),
    Err((T, Vec<Error>)),
    Panic(Vec<Error>),
}

pub struct Parser {
    lexer: Lexer,
    errors: Vec<Error>,
}

impl Parser {
    pub fn new(src: &str) -> Self {
        Self {
            lexer: Lexer::new(src),
            errors: Vec::new(),
        }
    }

    pub fn parse(mut self) -> ParserResult<Block> {
        let block = self.parse_block();
        ParserResult::Panic(())
    }

    // pub fn current(&self) -> Option<&TokenReference> {
    //     match self.lexer.current() {
    //         Some(LexerResult::Ok(token) | LexerResult::Recovered(token, _)) => Some(token),
    //         Some(LexerResult::Fatal(_)) => None,
    //         None => unreachable!("current() called past EOF"),
    //     }
    // }
    //
    // pub fn consume(&mut self) -> ParserResult<TokenReference> {
    //     let token = self.lexer.consume();
    //
    //     match token {
    //         Some(LexerResult::Ok(token)) => ParserResult::Ok(token),
    //         Some(LexerResult::Recovered(token, errors)) => {
    //             for error in errors {
    //                 self.errors.push(crate::Error::LexerError(error));
    //             }
    //
    //             ParserResult::Ok(token)
    //         }
    //         Some(LexerResult::Fatal(errors)) => {
    //             for error in errors {
    //                 self.errors.push(crate::Error::LexerError(error));
    //             }
    //
    //             ParserResult::Panic(errors)
    //         }
    //     }
    // }
    //
    // pub fn consume_if(&mut self, symbol: Symbol) -> Option<TokenReference> {
    //     match self.current() {
    //         // Some(token) => {
    //         //     // if token.is_symbol(symbol) {
    //         //     //     Some(self.consume().unwrap())
    //         //     // } else {
    //         //     //     None
    //         //     // }
    //         // }
    //         None => None,
    //     }
    // }
}

pub fn parse(src: &str) -> Result<bool, Box<Error>> {
    let mut parser = Parser::new(src);
    let block = parsers::parse_block(&mut parser);

    Ok(true)
}
