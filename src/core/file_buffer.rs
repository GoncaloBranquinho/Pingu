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
    // Processes the input
    fn execute(&mut self, cmd: Command, line: usize, col: usize) -> (usize, usize) {
        match cmd {
            Command::InsertChar(c) => {
                self.insert_char(line, col, c);
                (line, col + 1)
            }
            Command::NewLine => {
                self.insert_new_line(line, col);
                // For now it moves to the col 0, but in the future, it should be able to move to a
                // colunm based on the current block (such as a function)
                (line + 1, 0)
            }

            Command::DeleteChar => self.delete_char(line, col),
            _ => todo!(),
        }
    }

    // Gets all the Cells to be displayed on the terminal
    fn viewport(&self, mut line: usize, mut col: usize, mut size: usize) -> Vec<Vec<Cell>> {
        let mut res = Vec::new();

        while size > 0 {
            let row = match self.lines.get(line) {
                Some(row) => row,
                None => return res,
            };
            let mut line_cells = Vec::new();
            (col..col + min(size, row.len() - col)).for_each(|i| {
                line_cells.push(row[i]);
            });
            res.push(line_cells);
            size -= row.len() - col;
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
            let mut curr = self.lines.remove(line);
            let prev = self.lines.get_mut(line - 1).unwrap();
            let col = prev.len();
            prev.append(&mut curr);
            return (line - 1, col);
        }
        let row = self.lines.get_mut(line).unwrap();
        row.remove(col - 1);
        (line, col - 1)
    }

    fn insert_new_line(&mut self, line: usize, col: usize) {
        let row = self.lines.get_mut(line).unwrap();
        let right = row.drain(col..).collect();
        self.lines.insert(line + 1, right);
    }
}
