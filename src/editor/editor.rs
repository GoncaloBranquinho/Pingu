use std::collections::HashMap;

use std::io::stdout;
use std::{
    io::{Result, Stdout},
    time::Duration,
};

use crossterm::cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition};
use crossterm::event::{
    DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
    EnableFocusChange, EnableMouseCapture, poll, read,
};
use crossterm::execute;
use crossterm::terminal::{
    Clear, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size,
};

use crate::core::{BufferKind, Cursor, FileBuffer, Mode, Window};
use crate::input::ModeHandler;
use crate::render::Render;

#[derive(Debug)]
pub struct Editor {
    pub buffers: HashMap<usize, BufferKind>,
    pub windows: HashMap<usize, Window>,
    pub current_window: usize,
    pub mode: Mode,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            windows: HashMap::new(),
            current_window: 1,
            mode: Mode::Insert, // Should be Normal
        }
    }

    pub fn process_input(&mut self, stdout: &mut Stdout) -> Result<bool> {
        loop {
            if poll(Duration::from_millis(500))? {
                ModeHandler::handle(self, read()?);
                Render::render(self, stdout);
            } else {
                continue;
            }
        }
    }

    pub fn initialize_editor(&mut self) -> Result<()> {
        let file_buffer = BufferKind::File(FileBuffer::new(("untitled").to_string()));
        self.buffers.insert(1, file_buffer);
        let (width, height) = size()?;
        self.windows
            .insert(1, Window::new(1, width, height, 0, 0, 0));
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
            DisableBlinking,
            SavePosition,
            MoveTo(0, 0),
        )?;
        Ok(())
    }

    pub fn insert_char(&self, c: char) {
        todo!()
    }

    pub fn insert_newline(&self) {
        todo!()
    }

    pub fn delete_char(&self) {
        todo!()
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = disable_raw_mode();

        let mut stdout = stdout();
        let _ = execute!(
            stdout,
            EnableBlinking,
            DisableMouseCapture,
            DisableBracketedPaste,
            DisableFocusChange,
            RestorePosition,
            LeaveAlternateScreen,
        );
    }
}
