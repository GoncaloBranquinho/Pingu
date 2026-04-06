use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read},
    terminal::size,
};
use pingu::{editor::Editor, input::ModeHandler, render::Render};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let (term_width, term_height) = size()?;
    let mut editor = Editor::new(term_width as usize, term_height as usize);
    editor.initialize_terminal(&mut stdout)?;
    editor.initialize_editor()?;
    let mut renderer = Render::new(term_width as usize, term_height as usize);

    loop {
        if poll(Duration::from_millis(500))? {
            let command = ModeHandler::handle(&mut editor, read()?);
            editor.execute(command);
            renderer.render(&mut editor, &mut stdout)?;
            stdout.flush()?;
        } else {
            continue;
        }
    }
}
