use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::{core::Cell, editor::Editor};
use std::io::{self, Stdout, Write};

#[derive(Debug)]
pub struct Render {
    pub old_screen: Vec<Vec<Cell>>,
}

impl Render {
    pub fn new(term_width: usize, term_height: usize) -> Self {
        let new_screen: Vec<Vec<Cell>> = vec![vec![Cell::new(' '); term_width]; term_height];
        Self {
            old_screen: new_screen,
        }
    }

    pub fn render(&mut self, editor: &mut Editor, stdout: &mut Stdout) -> io::Result<()> {
        let mut new_screen: Vec<Vec<Cell>> =
            vec![vec![Cell::new(' '); editor.term_width]; editor.term_height];
        for window_id in &editor.active_windows {
            let window = editor.get_window(*window_id);
            let buffer = editor.get_buffer(window.buffer_id);
            let (line, col) = window.get_buffer_start();
            let cells = buffer.viewport(line, col, window.width * window.height);
            let mut i = window.x;
            let mut j = window.y;
            for row_cells in cells.iter() {
                for cell in row_cells.iter() {
                    new_screen[i][j] = *cell;
                    j += 1;
                    if j >= window.x + window.width {
                        j = window.y;
                        i += 1;
                    }
                }
                i += 1;
                j = window.y;
            }
        }

        if self.old_screen.len() != new_screen.len()
            || self.old_screen[0].len() != new_screen[0].len()
        {
            execute!(stdout, Clear(ClearType::All))?;
            self.old_screen = vec![vec![Cell::new('\0'); new_screen[0].len()]; new_screen.len()];
        }

        (0..new_screen.len()).for_each(|i| {
            (0..new_screen[i].len()).for_each(|j| {
                if self.old_screen[i][j] != new_screen[i][j] {
                    let _ = queue!(stdout, MoveTo(j as u16, i as u16), Print(new_screen[i][j]));
                }
            });
        });

        self.old_screen = new_screen;

        let active_window = editor.get_window(editor.current_window);
        let (line, col) = active_window.window_cursor();
        queue!(stdout, MoveTo(col as u16, line as u16))?;
        Ok(())
    }
}
