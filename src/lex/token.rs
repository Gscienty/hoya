use super::{Location, Position};
use std::string::String;

#[derive(Clone)]
pub struct Token {
    token_type: String,
    token_value: String,
    token_location: Option<Location>,
}

impl Token {
    pub fn new(token_type: &str, token_value: &str) -> Self {
        Token {
            token_type: String::from(token_type),
            token_value: String::from(token_value),
            token_location: None,
        }
    }

    pub fn set_location(&mut self, begin_position: Position, end_position: Position) {
        self.token_location = Some(Location::new(begin_position, end_position));
    }

    pub fn get_type(&self) -> &str {
        self.token_type.as_str()
    }

    pub fn get_value(&self) -> &str {
        self.token_value.as_str()
    }

    pub fn get_location(&self) -> Location {
        self.token_location
            .or_else(|| Some(Location::new(Position::new_zero(), Position::new_zero())))
            .unwrap()
    }
}
