use fuse_ast::Chunk;

pub struct Semantic<'a> {
    source: &'a str,
    chunk: Chunk,
}

impl<'a> Semantic<'a> {
    pub fn new(source: &'a str, chunk: Chunk) -> Self {
        Self { source, chunk }
    }
}
