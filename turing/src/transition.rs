use crate::direction::Direction;
use crate::states::State;
use std::fmt::{Display, Formatter, Result};

pub struct Transition<'a> {
    pub current_state: &'a State,
    pub current_char: char,
    pub new_state: &'a State,
    pub new_char: char,
    pub direction: Direction,
}

impl<'a> Display for Transition<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} -> {}|{}|{:?} -> {}",
            self.current_state.name,
            self.current_char,
            self.new_char,
            self.direction,
            self.new_state.name
        )
    }
}
