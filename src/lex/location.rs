use super::Position;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Location {
    begin: Position,
    end: Position,
}

impl Location {
    pub fn new(begin: Position, end: Position) -> Self {
        Location { begin, end }
    }

    pub fn get_begin(&self) -> Position {
        self.begin
    }

    pub fn get_end(&self) -> Position {
        self.end
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Location(begin: {}, end: {})", self.begin, self.end)
    }
}
