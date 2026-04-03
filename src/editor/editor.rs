use std::collections::HashMap;

use std::{
    io::{Result, Stdout},
    time::Duration,
};

use crossterm::event::{EnableBracketedPaste, EnableFocusChange, EnableMouseCapture, poll, read};
use crossterm::execute;
use crossterm::terminal::enable_raw_mode;

use crate::core::{BufferKind, Cursor, Mode, Window};
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
            buffers: HashMap::new(), // TODO Initialize with default buffers
            windows: HashMap::new(), // TODO Initialize with default windows
            current_window: 1,
            mode: Mode::Normal,
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

    pub fn initialize(&self, stdout: &mut Stdout) -> Result<()> {
        enable_raw_mode()?;
        execute!(
            stdout,
            EnableBracketedPaste,
            EnableFocusChange,
            EnableMouseCapture
        )?;
        Ok(())
    }
}
