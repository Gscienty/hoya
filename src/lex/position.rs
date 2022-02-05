use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }

    pub fn new_zero() -> Self {
        Position::new(0, 0)
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_column(&self) -> usize {
        self.column
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.line < other.line || (self.line == other.line && self.column < other.column) {
            Some(Ordering::Less)
        } else if self.line > other.line || (self.line == other.line && self.column > other.column)
        {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Position(line: {}, column: {})", self.line, self.column)
    }
}
