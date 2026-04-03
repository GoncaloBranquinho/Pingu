use crate::core::Cursor;

#[derive(Debug)]
pub struct Window {
    pub buffer_id: usize,
    pub cursor: Cursor,
    pub line_offset: usize,
    pub col_offset: usize,
    pub width: u16,
    pub height: u16,
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Window {
    pub fn new(buffer_id: usize, width: u16, height: u16, x: usize, y: usize, z: usize) -> Self {
        Window {
            buffer_id,
            cursor: Cursor::new(),
            line_offset: 0,
            col_offset: 0,
            width,
            height,
            x,
            y,
            z,
        }
    }

    pub fn buffer_cursor(&self) -> (usize, usize) {
        (
            self.cursor.line() + self.line_offset,
            self.cursor.col() + self.col_offset,
        )
    }
}
