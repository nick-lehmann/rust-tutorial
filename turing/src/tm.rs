use std::collections::HashMap;

use crate::direction::*;
use crate::states::*;
use crate::tape::*;
use crate::transition::*;

pub struct TuringMachine<'a> {
    pub tape: Tape,
    pub transitions: Vec<Transition<'a>>,
}

impl<'a> TuringMachine<'a> {
    pub fn new(tape: Tape, transitions: Vec<Transition<'a>>) -> Self {
        TuringMachine { tape, transitions }
    }

    pub fn run(&mut self) {
        let mut current_state = self.get_start_state().unwrap();
        let mut step_counter = 0;

        while current_state.state_type != StateType::Final && step_counter < 1000 {
            let current_token = self.tape.read();

            let transition = self
                .transitions
                .iter()
                .find(|transition| {
                    transition.current_state == current_state
                        && transition.current_char == current_token
                })
                .expect(&format!(
                    "There was no transition found for state {} and current token {}",
                    current_state.name, current_token
                ));

            println!("Step: {}", step_counter);
            println!("State: {}", current_state.name);
            println!("Tape: {}", self.tape);
            println!("Head: {}", self.tape.head);
            println!("Transition: {}", transition);
            println!("======================");

            self.tape.write(transition.new_char);
            self.tape.move_head(&transition.direction);
            current_state = transition.new_state;

            step_counter += 1;
        }
    }

    fn get_start_state(&self) -> Option<&State> {
        for transition in &self.transitions {
            if transition.current_state.state_type == StateType::Start {
                return Some(transition.current_state);
            }
            if transition.new_state.state_type == StateType::Start {
                return Some(transition.new_state);
            }
        }
        return None;
    }
}
