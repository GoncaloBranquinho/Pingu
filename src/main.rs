use std::io::{self, Write};

use pingu::editor::Editor;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut editor = Editor::new();
    editor.initialize_terminal(&mut stdout)?;
    editor.initialize_editor()?;
    editor.process_input(&mut stdout)?;
    stdout.flush()?;
    Ok(())
}
