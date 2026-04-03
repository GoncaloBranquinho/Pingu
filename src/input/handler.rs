use crossterm::event::{Event, KeyCode};

use crate::{core::Mode, editor::Editor};

pub struct ModeHandler;

impl ModeHandler {
    pub fn handle(editor: &mut Editor, event: Event) {
        match editor.mode {
            Mode::Normal => Self::normal(editor, event),
            Mode::Insert => Self::insert(editor, event),
            Mode::Command => Self::command(editor, event),
        }
    }

    fn normal(editor: &mut Editor, event: Event) {
        match event {
            Event::Key(key) => todo!(),
            _ => todo!(),
        }
    }

    fn insert(editor: &mut Editor, event: Event) {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char(c) => {
                    editor.insert_char(c);
                }
                KeyCode::Enter => {
                    editor.insert_newline();
                }
                KeyCode::Backspace => {
                    editor.delete_char();
                }
                KeyCode::Esc => {
                    editor.mode = Mode::Normal;
                }
                _ => {}
            },
            _ => todo!(),
        }
    }

    fn command(editor: &mut Editor, event: Event) {}
}
