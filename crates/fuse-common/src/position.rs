#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub bytes: usize,
    pub line: usize,
    pub character: usize,
}

impl Position {
    pub fn new(bytes: usize, line: usize, character: usize) -> Self {
        Self {
            bytes,
            line,
            character,
        }
    }
}
