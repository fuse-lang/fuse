mod cursor;
mod lexer;
mod parsers;

/// Maximum length of source that can be parsed in bytes.
/// ~4GiB on 64-bit systems, ~2 GiB on 32-bit systems.
// 64-bit limit is bound to the `Span` fields which are `u32`.
// 32-bit limit is bound to `isize::MAX` because of allocator limits,
// we are planning to use custom allocators in the future.
pub const MAX_SOURCE_SIZE: usize = if std::mem::size_of::<usize>() >= 8 {
    // 64-bit systems
    u32::MAX as usize
} else {
    // 32-bit or 16-bit systems
    isize::MAX as usize
};

pub struct ParsedChunk {
    pub chunk: Option<fuse_ast::Chunk>,
    pub errors: Vec<Error>,
    pub paniced: bool,
}

impl ParsedChunk {
    fn new(chunk: fuse_ast::Chunk, errors: Vec<Error>) -> Self {
        Self {
            chunk: Some(chunk),
            errors,
            paniced: false,
        }
    }

    fn with_panic(errors: Vec<Error>) -> Self {
        Self {
            chunk: None,
            errors,
            paniced: true,
        }
    }
}

pub enum Error {
    LexerError(lexer::LexerError),
}

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
    errors: Vec<Error>,
    source: &'a str,
    factory: fuse_ast::AstFactory,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: lexer::Lexer::new(source),
            errors: Vec::new(),
            source,
            factory: fuse_ast::AstFactory(),
        }
    }

    pub fn parse(mut self) -> ParsedChunk {
        match self.parse_chunk() {
            ParserResult::Ok(chunk) => ParsedChunk::new(chunk, self.errors),
            ParserResult::Err | ParserResult::NotFound => ParsedChunk::with_panic(self.errors),
        }
    }
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

pub fn parse<'a>(src: &'a str) -> ParsedChunk {
    Parser::new(src).parse()
}
