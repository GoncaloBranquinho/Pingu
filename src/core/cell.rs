use core::fmt::{Formatter, Result};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    value: char,
}

impl Cell {
    pub fn new(value: char) -> Self {
        Self { value }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}
