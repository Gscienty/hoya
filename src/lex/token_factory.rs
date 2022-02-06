use super::{StateChange, Token};

pub struct TokenFactory<'t> {
    token_type: &'t str,

    increment_offset: Option<usize>,
    increment_lines: Option<usize>,
    next_state: Option<&'static str>,
}

impl<'t> TokenFactory<'t> {
    pub fn new(token_type: &'t str) -> Self {
        TokenFactory {
            token_type,

            increment_offset: None,
            increment_lines: None,
            next_state: None,
        }
    }

    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.increment_offset = Some(offset);

        self
    }

    pub fn lines(&mut self, lines: usize) -> &mut Self {
        self.increment_lines = Some(lines);

        self
    }

    pub fn next_state(&mut self, state: &'static str) -> &mut Self {
        self.next_state = Some(state);

        self
    }

    pub fn build(&self, token: &str) -> (Token, StateChange) {
        (
            Token::new(self.token_type, token),
            StateChange::new(
                self.increment_offset.unwrap_or(token.len()),
                self.increment_lines.unwrap_or(0),
                self.next_state,
            ),
        )
    }
}
