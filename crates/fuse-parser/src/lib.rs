mod cursor;
mod lexer;
mod parsers;

pub enum Error {
    LexerError(lexer::LexerError),
}

pub enum ParserResult<T> {
    Ok(T),
    Err,
    NotFound,
}

impl<T> ParserResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            ParserResult::Ok(result) => result,
            ParserResult::Err => panic!("Attempt to unwrap a ParserResult::Err."),
            ParserResult::NotFound => panic!("Attempt to unwrap a ParserResult::NotFound."),
        }
    }
}

pub struct Parser {
    lexer: lexer::Lexer,
    errors: Vec<Error>,
}

impl Parser {
    pub fn new(src: &str) -> Self {
        Self {
            lexer: lexer::Lexer::new(src),
            errors: Vec::new(),
        }
    }

    pub fn parse(mut self) -> ParserResult<fuse_ast::Block> {
        let block = self.parse_block();
        ParserResult::NotFound
    }
}

pub fn parse(src: &str) -> Result<bool, Box<Error>> {
    let mut parser = Parser::new(src);
    let block = parser.parse();

    Ok(true)
}
