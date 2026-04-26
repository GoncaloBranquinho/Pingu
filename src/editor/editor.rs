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
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, SetSize, disable_raw_mode, enable_raw_mode,
};
use crossterm::{execute, queue};

use crate::commands::Command;
use crate::core::{Buffer, FileBuffer, Mode, Window, window};

#[derive(Debug)]
pub struct Editor {
    pub buffers: HashMap<usize, Box<dyn Buffer>>,
    pub windows: HashMap<usize, Window>,
    //pub str_to_id: HashMap<String, usize>,
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

    pub fn get_buffer(&self, id: usize) -> &dyn Buffer {
        self.buffers.get(&id).unwrap().as_ref()
    }

    pub fn get_pair(&mut self, id: usize) -> (&mut dyn Buffer, &mut Window) {
        let window = self.windows.get_mut(&id).unwrap();
        let buffer_id = window.buffer_id;
        let buffer = self.buffers.get_mut(&buffer_id).unwrap().as_mut();
        (buffer, window)
    }

    pub fn execute(&mut self, command: Command, stdout: &mut Stdout) {
        let (buffer, window) = self.get_pair(self.current_window);
        let (line, col) = window.buffer_cursor();
        match command {
            Command::ExitInsertMode => {
                self.mode = Mode::Normal;
                let _ = queue!(stdout, SetCursorStyle::SteadyBlock);
            }

            Command::EnterInsertMode => {
                self.mode = Mode::Insert;
                let _ = queue!(stdout, SetCursorStyle::SteadyBar);
            }

            Command::Resize(width, height) => {
                let _ = queue!(stdout, SetSize(width, height));
                self.term_width = width as usize;
                self.term_height = height as usize;
                self.distribute_sizes(width, height);
                let window_ids: Vec<usize> = self.windows.keys().copied().collect();
                for window_id in window_ids {
                    let (aux_buffer, aux_window) = self.get_pair(window_id);
                    aux_window.update_lines_before(aux_buffer);
                    aux_window.update_cursor(line, col);
                }
            }

            Command::MoveUp | Command::MoveDown | Command::MoveLeft | Command::MoveRight => {
                let (new_line, new_col) = buffer.execute(command, line, col);
                window.update_cursor(new_line, new_col);
            }
            Command::DeleteRightChar => {
                let (_, _) = buffer.execute(command, line, col);
                window.update_lines_before(buffer);
            }
            _ => {
                let (new_line, new_col) = buffer.execute(command, line, col);
                window.update_lines_before(buffer);
                window.update_cursor(new_line, new_col);
                /* This is how we can print the lines
                let (term_line, term_col) = window.window_cursor();
                queue!(
                    stdout,
                    MoveTo(0, term_line as u16),
                    Print(term_line + window.first_line_window as isize + 1)
                ); */
            }
        }
    }

    pub fn distribute_sizes(&mut self, width: u16, height: u16) {
        let window_ids: Vec<usize> = self.windows.keys().copied().collect();
        for window_id in window_ids {
            let (buffer, window) = self.get_pair(window_id);
            window.height = height as usize;
            window.width = width as usize;
            window.update_lines_before(buffer);
            let (line, col) = window.buffer_cursor();
            window.update_cursor(line, col);
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
