use std::io::{self, Write};

use pingu::editor::Editor;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut editor = Editor::new();
    editor.initialize(&mut stdout)?;
    editor.process_input(&mut stdout)?;
    stdout.flush()?;
    Ok(())
}
