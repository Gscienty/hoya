pub struct StateChange {
    increment_offset: usize,
    increment_lines: usize,

    next_state: Option<&'static str>,
}

impl StateChange {
    pub fn new(
        increment_offset: usize,
        increment_lines: usize,
        next_state: Option<&'static str>,
    ) -> Self {
        StateChange {
            increment_offset,
            increment_lines,
            next_state,
        }
    }

    pub fn get_increment_offset(&self) -> usize {
        self.increment_offset
    }

    pub fn get_increment_lines(&self) -> usize {
        self.increment_lines
    }

    pub fn get_next_state(&self) -> Option<&str> {
        self.next_state
    }
}
