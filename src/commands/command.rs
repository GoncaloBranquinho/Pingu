#[derive(Debug)]
pub enum Command {
    InsertChar(char),
    NewLine,
    DeleteChar,
    ExitInsertMode,
}
