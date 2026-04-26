#[derive(Debug)]
pub enum Command {
    InsertChar(char),
    NewLine,
    ExitInsertMode,
    EnterInsertMode,
    Resize(u16, u16),
    DeleteRightChar,
    DeleteLeftChar,
    MoveRight,
    MoveLeft,
    MoveDown,
    MoveUp,
}
