use std::cmp::min;

use crate::{
    commands::Command,
    core::{Buffer, Cell},
};

#[derive(Debug)]
pub struct FileBuffer {
    lines: Vec<Vec<Cell>>,
    name: String,
}

impl Buffer for FileBuffer {
    fn execute(&mut self, cmd: Command, line: usize, col: usize) -> (usize, usize) {
        match cmd {
            Command::InsertChar(c) => {
                self.insert_char(line, col, c);
                (line, col + 1)
            }
            Command::DeleteChar => self.delete_char(line, col),
            _ => todo!(),
        }
    }

    fn viewport(&self, mut line: usize, mut col: usize, mut size: usize) -> Vec<Vec<Cell>> {
        let mut res = Vec::new();

        while size > 0 {
            let row = match self.lines.get(line) {
                Some(row) => row,
                None => break,
            };

            let remaining = row.len() - col;
            let remaining = min(remaining, size);

            let mut line_cells = Vec::new();

            (col..col + remaining).for_each(|i| {
                line_cells.push(row[i]);
            });
            res.push(line_cells);
            size -= remaining;
            line += 1;
            col = 0;
        }
        res
    }

    fn count_lines(&self) -> usize {
        self.lines.len()
    }

    fn line_size(&self, line: usize) -> usize {
        self.lines.get(line).unwrap().len()
    }
}

impl FileBuffer {
    pub fn new(name: String) -> Self {
        Self {
            lines: vec![Vec::new()],
            name,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    pub fn insert_char(&mut self, line: usize, col: usize, c: char) {
        let line = self.lines.get_mut(line).unwrap();
        line.insert(col, Cell::new(c));
    }

    fn insert_word(&mut self, line: usize, col: usize, word: &str) {
        for c in word.chars() {
            self.insert_char(line, col, c);
        }
    }

    fn delete_char(&mut self, line: usize, col: usize) -> (usize, usize) {
        if col == 0 {
            if line == 0 {
                return (0, 0);
            }
            let curr = self.lines.remove(line);
            let prev = self.lines.get_mut(line - 1).unwrap();
            let col = prev.len();
            prev.extend(curr);
            return (line - 1, col);
        }
        let row = self.lines.get_mut(line).unwrap();
        row.remove(col - 1);
        (line, col - 1)
    }
}
