#[derive(Debug)]
pub struct Cursor {
    line: usize,
    col: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { line: 0, col: 0 }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn set_col(&mut self, col: usize) {
        self.col = col;
    }

    pub fn set_line(&mut self, line: usize) {
        self.line = line;
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}
