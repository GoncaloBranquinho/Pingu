use crate::{commands::Command, core::Cell};
use std::fmt::Debug;

pub trait Buffer: Debug {
    fn execute(&mut self, cmd: Command, line: usize, col: usize) -> (usize, usize);
    fn count_lines(&self) -> usize;
    fn line_size(&self, line: usize) -> usize;
    fn viewport(&self, line: usize, col: usize, size: usize) -> Vec<Vec<Cell>>;
}
