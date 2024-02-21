mod cursor;
mod error;
mod lexer;
mod parsers;

/// Maximum length of source that can be parsed in bytes.
/// ~4GiB on 64-bit systems, ~2 GiB on 32-bit systems.
// 64-bit limit is bound to the `Span` fields which are `u32`.
// 32-bit limit is bound to `isize::MAX` because of allocator limits,
// We are planning to use custom allocators in the future.
pub const MAX_SOURCE_SIZE: usize = if std::mem::size_of::<usize>() >= 8 {
    // 64-bit systems
    u32::MAX as usize
} else {
    // 32-bit or 16-bit systems
    isize::MAX as usize
};

pub struct ParsedChunk {
    pub chunk: Option<fuse_ast::Chunk>,
    pub errors: Vec<error::Error>,
    pub paniced: bool,
}

impl ParsedChunk {
    fn new(chunk: fuse_ast::Chunk, errors: Vec<error::Error>) -> Self {
        Self {
            chunk: Some(chunk),
            errors,
            paniced: false,
        }
    }

    fn with_panic(errors: Vec<error::Error>) -> Self {
        Self {
            chunk: None,
            errors,
            paniced: true,
        }
    }
}

pub struct LazyParser<'a>(&'a str);

impl<'a> LazyParser<'a> {
    pub fn parse(mut self) -> ParsedChunk {
        Parser::new(self.0).parse()
    }
}

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
    errors: Vec<error::Error>,
    source: &'a str,
    ast: fuse_ast::AstFactory,
    prev_token_end: u32,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: lexer::Lexer::new(source),
            errors: Vec::new(),
            source,
            ast: fuse_ast::AstFactory(),
            prev_token_end: 0,
        }
    }

    pub fn new_lazy(source: &'a str) -> LazyParser<'a> {
        LazyParser(source)
    }

    pub fn parse(mut self) -> ParsedChunk {
        match self.parse_chunk() {
            ParserResult::Ok(chunk) => ParsedChunk::new(chunk, self.errors),
            ParserResult::Err(error) => {
                self.push_error(error);
                ParsedChunk::with_panic(self.errors)
            }
        }
    }

    fn push_error<E: Into<error::Error>>(&mut self, error: E) {
        self.errors.push(error.into());
    }

    fn push_errors<E: Into<error::Error>>(&mut self, errors: Vec<E>) {
        self.errors
            .append(&mut errors.into_iter().map(E::into).collect())
    }

    fn start_span(&self) -> fuse_common::Span {
        let token = self.cur_token();
        fuse_common::Span::new(token.start(), 0)
    }

    fn end_span(&self, span: fuse_common::Span) -> fuse_common::Span {
        let mut span = span;
        span.end = 0;
        span
    }
}

pub type ParserResult<T> = Result<T, error::Error>;

pub fn parse<'a>(src: &'a str) -> ParsedChunk {
    Parser::new(src).parse()
}
