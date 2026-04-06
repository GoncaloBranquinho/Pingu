use crate::core::{Buffer, Cursor};

#[derive(Debug)]
pub struct Window {
    pub buffer_id: usize,
    pub cursor: Cursor,
    pub width: usize,
    pub height: usize,
    // It represents the first line of the buffer that is currently
    // displayed on the screen
    pub first_line_buffer: usize,
    // It represents the first line of the window, with wrapped lines included
    pub first_line_window: usize,
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub lines_before: Vec<usize>,
}

impl Window {
    pub fn new(
        buffer_id: usize,
        width: usize,
        height: usize,
        x: usize,
        y: usize,
        z: usize,
    ) -> Self {
        Window {
            buffer_id,
            cursor: Cursor::new(),
            width,
            height,
            first_line_buffer: 0,
            first_line_window: 0,
            x,
            y,
            z,
            lines_before: Vec::new(),
        }
    }

    pub fn buffer_cursor(&self) -> (usize, usize) {
        (self.cursor.line(), self.cursor.col())
    }

    // gets the current window cursor, to be displayed on the terminal
    pub fn window_cursor(&self) -> (usize, usize) {
        let (buffer_line, buffer_col) = self.buffer_cursor();
        let offset = buffer_col / self.width;
        let window_line = self.x
            + (self.lines_before[buffer_line] - self.lines_before[self.first_line_buffer])
            + offset;
        let window_col = self.y + (buffer_col % self.width);
        (window_line, window_col)
    }

    // helper to get the start of the viewport on the buffer
    pub fn get_buffer_start(&self) -> (usize, usize) {
        let mut l = 0;
        let mut r = self.lines_before.len() - 1;
        let mut mid = (l + r) / 2;
        while l <= r {
            mid = (l + r) / 2;
            // Is this case even possible to happen??
            if mid == self.lines_before.len() - 1 {
                break;
            }
            if self.lines_before[mid] <= self.first_line_window
                && self.first_line_window < self.lines_before[mid + 1]
            {
                break;
            } else if self.first_line_window < self.lines_before[mid] {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        (
            mid,
            (self.first_line_window - self.lines_before[mid]) * self.width,
        )
    }

    // updates the count of wrapped lines before each buffer line
    pub fn update_lines_before(&mut self, buffer: &dyn Buffer) {
        self.lines_before.clear();
        self.lines_before.push(0);

        for line in 0..buffer.count_lines() {
            let len = buffer.line_size(line);
            let wraps = if len == 0 {
                1
            } else {
                len.div_ceil(self.width)
            };
            let prev = *self.lines_before.last().unwrap();
            self.lines_before.push(prev + wraps);
        }
    }
}
