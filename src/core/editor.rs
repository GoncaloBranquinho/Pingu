use std::collections::HashMap;

use std::{
    io::{Result, Stdout},
    time::Duration,
};

use crossterm::event::{Event, Event::*, poll, read};

use crate::core::{BufferKind, Cursor, Mode, Window};

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

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn buffer(&mut self) -> &mut BufferKind {
        let buffer_id = self.windows.get(&self.current_window).unwrap().buffer_id;
        self.buffers.get_mut(&buffer_id).unwrap()
    }

    pub fn buffer_cursor(&self) -> (usize, usize) {
        self.windows
            .get(&self.current_window)
            .unwrap()
            .buffer_cursor()
    }

    pub fn window_cursor(&mut self) -> &mut Cursor {
        &mut self.windows.get_mut(&self.current_window).unwrap().cursor
    }

    pub fn process_input(&mut self, stdout: &mut Stdout) -> Result<bool> {
        loop {
            if poll(Duration::from_millis(500))? {
                self.handle(read()?);
            } else {
                todo!()
            }
        }
    }

    pub fn handle(&mut self, event: Event) {
        match event {
            Key(key) => {
                let (line, col) = self.buffer_cursor();
                let buffer = self.buffer();
                buffer.insert_char(line, col, key.code.as_char().unwrap());
            }
            Mouse(mouse) => {
                todo!()
            }
            Paste(paste) => {
                todo!()
            }
            Resize(width, height) => {
                todo!()
            }
            FocusGained => {
                todo!()
            }
            FocusLost => {
                todo!()
            }
        }
    }
}
