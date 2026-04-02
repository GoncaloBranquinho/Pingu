use std::io::{self, Write};

use crossterm::{
    ExecutableCommand,
    event::{Event, KeyCode, read},
    terminal::{self, enable_raw_mode},
};
use pingu::core::Editor;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut editor = Editor::new();
    editor.process_input(&mut stdout);
    stdout.flush()?;
    Ok(())
}
