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
            Event::Resize(width, height) => Command::Resize(width, height),
            _ => todo!(),
        }
    }

    fn insert(editor: &mut Editor, event: Event) -> Command {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char(c) => Command::InsertChar(c),
                KeyCode::Enter => Command::NewLine,
                KeyCode::Backspace => Command::DeleteLeftChar,
                KeyCode::Delete => Command::DeleteRightChar,
                KeyCode::Esc => Command::ExitInsertMode,
                KeyCode::Up => Command::MoveUp,
                KeyCode::Down => Command::MoveDown,
                KeyCode::Left => Command::MoveLeft,
                KeyCode::Right => Command::MoveRight,
                _ => todo!(),
            },

            c => Command::EnterInsertMode,
        }
    }

    fn command(editor: &mut Editor, event: Event) -> Command {
        todo!()
    }
}
