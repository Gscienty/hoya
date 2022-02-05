pub struct IncrementOffset {
    increment_offset: usize,
    increment_lines: usize,
}

impl IncrementOffset {
    pub fn new(offset: usize, lines: usize) -> Self {
        IncrementOffset {
            increment_offset: offset,
            increment_lines: lines,
        }
    }

    pub fn get_increment_offset(&self) -> usize {
        self.increment_offset
    }

    pub fn get_increment_lines(&self) -> usize {
        self.increment_lines
    }
}
