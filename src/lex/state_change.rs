#[derive(Clone, Copy)]
pub enum NextStateChange {
    Push(&'static str),
    Pop(usize),
}

pub struct StateChange {
    increment_offset: usize,
    increment_lines: usize,

    next_state: Vec<NextStateChange>,
}

impl StateChange {
    pub fn new(
        increment_offset: usize,
        increment_lines: usize,
        next_state: Vec<NextStateChange>,
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

    pub fn get_next_state(&self) -> &Vec<NextStateChange> {
        &self.next_state
    }
}
