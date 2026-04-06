use crossterm::event::{Event, KeyCode};

use crate::{commands::Command, core::Mode, editor::Editor};

pub struct ModeHandler;

impl ModeHandler {
    pub fn handle(editor: &mut Editor, event: Event) -> Command {
        match editor.mode {
            Mode::Normal => Self::normal(editor, event),
            Mode::Insert => Self::insert(editor, event),
            Mode::Command => Self::command(editor, event),
        }
    }

    fn normal(editor: &mut Editor, event: Event) -> Command {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('i') => Command::EnterInsertMode,
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn insert(editor: &mut Editor, event: Event) -> Command {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char(c) => Command::InsertChar(c),
                KeyCode::Enter => Command::NewLine,
                KeyCode::Backspace => Command::DeleteChar,
                KeyCode::Esc => Command::ExitInsertMode,
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn command(editor: &mut Editor, event: Event) -> Command {
        todo!()
    }
}
