use crate::core::Cell;

#[derive(Debug)]
pub struct FileBuffer {
    lines: Vec<Vec<Cell>>,
    name: String,
}

impl FileBuffer {
    pub fn new(name: String) -> Self {
        Self {
            lines: vec![Vec::new()],
            name,
        }
    }
}
