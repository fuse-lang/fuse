use super::token::Position;

pub struct Source {
    source: Vec<char>,
    position: Position,
    index: usize,
}

impl Source {
    pub fn new(src: &str) -> Self {
        Self {
            source: src.chars().collect(),
            position: Position::new(1, 1, 0),
            index: 0,
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    fn current(&self) -> Option<char> {
        self.source.get(self.index + 1).copied()
    }

    pub(crate) fn next(&mut self) -> Option<char> {
        let next = self.current()?;

        if next == '\n' {
            self.position.line += 1;
            self.position.character = 1;
        } else {
            self.position.character += 1;
        }

        self.position.bytes += next.len_utf8();
        self.index += 1;

        Some(next)
    }

    pub(crate) fn consume_if(&mut self, character: char) -> bool {
        if self.current() == Some(character) {
            self.next();
            true
        } else {
            false
        }
    }
}
