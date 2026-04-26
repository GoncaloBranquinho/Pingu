use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read},
    terminal::size,
};
use pingu::{commands::Command, editor::Editor, input::ModeHandler, render::Render};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let (term_width, term_height) = size()?;
    let mut editor = Editor::new(term_width as usize, term_height as usize);
    editor.initialize_terminal(&mut stdout)?;
    editor.initialize_editor()?;
    let mut renderer = Render::new(term_width as usize, term_height as usize);

    let mut last_size = size()?;

    loop {
        if poll(Duration::from_millis(500))? {
            let event = read()?;

            let command = ModeHandler::handle(&mut editor, event);
            editor.execute(command, &mut stdout);

            renderer.render(&mut editor, &mut stdout)?;
            stdout.flush()?;
        } else {
            let curr_size = size()?;
            if curr_size != last_size {
                last_size = curr_size;
                let command = Command::Resize(curr_size.0, curr_size.1);
                editor.execute(command, &mut stdout);
                renderer.render(&mut editor, &mut stdout)?;
                stdout.flush()?;
            }
        }
    }
}
