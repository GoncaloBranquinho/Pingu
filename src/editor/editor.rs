use std::collections::HashMap;

use std::io::stdout;
use std::io::{Result, Stdout};

use crossterm::cursor::{
    DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition, SetCursorStyle,
};
use crossterm::event::{
    DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
    EnableFocusChange, EnableMouseCapture,
};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

use crate::commands::Command;
use crate::core::{Buffer, FileBuffer, Mode, Window};

#[derive(Debug)]
pub struct Editor {
    pub buffers: HashMap<usize, Box<dyn Buffer>>,
    pub windows: HashMap<usize, Window>,
    pub active_windows: Vec<usize>,
    pub current_window: usize,
    pub mode: Mode,
    pub term_width: usize,
    pub term_height: usize,
}

impl Editor {
    pub fn new(term_width: usize, term_height: usize) -> Self {
        Self {
            buffers: HashMap::new(),
            windows: HashMap::new(),
            active_windows: Vec::new(),
            current_window: 1,
            mode: Mode::Insert,
            term_width,
            term_height,
        }
    }

    pub fn initialize_editor(&mut self) -> Result<()> {
        let file_buffer: Box<dyn Buffer> = Box::new(FileBuffer::new(("untitled").to_string()));
        let mut window = Window::new(1, self.term_width, self.term_height, 0, 0, 0);
        window.update_lines_before(file_buffer.as_ref());
        self.buffers.insert(1, file_buffer);
        self.windows.insert(1, window);
        self.active_windows.push(1);
        Ok(())
    }

    pub fn initialize_terminal(&self, stdout: &mut Stdout) -> Result<()> {
        enable_raw_mode()?;
        execute!(
            stdout,
            EnterAlternateScreen,
            EnableBracketedPaste,
            EnableFocusChange,
            EnableMouseCapture,
            SetCursorStyle::SteadyBar,
            DisableBlinking,
            SavePosition,
            MoveTo(0, 0),
        )?;
        Ok(())
    }

    pub fn get_window(&self, id: usize) -> &Window {
        self.windows.get(&id).unwrap()
    }

    pub fn get_window_as_mut(&mut self, id: usize) -> &mut Window {
        self.windows.get_mut(&id).unwrap()
    }

    pub fn get_buffer(&self, id: usize) -> &dyn Buffer {
        self.buffers.get(&id).unwrap().as_ref()
    }

    pub fn get_buffer_as_mut(&mut self, id: usize) -> &mut dyn Buffer {
        self.buffers.get_mut(&id).unwrap().as_mut()
    }

    pub fn execute(&mut self, command: Command) {
        let window = self.get_window(self.current_window);
        let buffer_id = window.buffer_id;
        let (line, col) = window.buffer_cursor();
        match command {
            Command::ExitInsertMode => {
                self.mode = Mode::Normal;
            }
            _ => {
                let (new_line, new_col) = self
                    .get_buffer_as_mut(buffer_id)
                    .execute(command, line, col);
                let window = self.get_window_as_mut(self.current_window);
                window.cursor.set_cursor(new_line, new_col);
            }
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = disable_raw_mode();

        let mut stdout = stdout();
        let _ = execute!(
            stdout,
            EnableBlinking,
            SetCursorStyle::DefaultUserShape,
            DisableMouseCapture,
            DisableBracketedPaste,
            DisableFocusChange,
            RestorePosition,
            LeaveAlternateScreen,
        );
    }
}
