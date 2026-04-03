use crate::core::{Cell, FileBuffer};

#[derive(Debug)]
pub enum BufferKind {
    File(FileBuffer),
    Ui,
}

impl BufferKind {
    fn name(&self) -> &str {
        todo!()
    }

    pub fn insert_char(&mut self, line: usize, col: usize, c: char) {
        todo!()
    }

    fn insert_word(&mut self, line: usize, col: usize, word: &str) {
        for c in word.chars() {
            self.insert_char(line, col, c);
        }
    }

    fn lines(&self) -> &Vec<Vec<Cell>> {
        todo!()
    }
}
