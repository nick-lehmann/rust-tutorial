#![allow(unused_imports, unused_variables)]

pub mod direction;
pub mod states;
pub mod tape;
pub mod tm;
pub mod transition;

#[cfg(test)]
mod tests {
    use crate::{
        direction::Direction,
        states::{State, StateType},
        tape::Tape,
        tm::TuringMachine,
        transition::Transition,
    };
    #[test]
    fn duplicate_characters() {
        let s1 = State::new(String::from("s1"), StateType::Start);
        let s2 = State::new(String::from("s2"), StateType::None);
        let s3 = State::new(String::from("s3"), StateType::None);
        let s4 = State::new(String::from("s4"), StateType::None);
        let s5 = State::new(String::from("s5"), StateType::None);
        let s6 = State::new(String::from("s6"), StateType::Final);

        let transitions = vec![
            Transition {
                current_state: &s1,
                current_char: '1',
                new_char: '0',
                new_state: &s2,
                direction: Direction::Right,
            },
            Transition {
                current_state: &s2,
                current_char: '0',
                new_char: '0',
                new_state: &s6,
                direction: Direction::None,
            },
            Transition {
                current_state: &s2,
                current_char: '1',
                new_char: '1',
                new_state: &s2,
                direction: Direction::Right,
            },
            Transition {
                current_state: &s2,
                current_char: '0',
                new_char: '0',
                new_state: &s3,
                direction: Direction::Right,
            },
            Transition {
                current_state: &s3,
                current_char: '1',
                new_char: '1',
                new_state: &s3,
                direction: Direction::Right,
            },
            Transition {
                current_state: &s3,
                current_char: '0',
                new_char: '1',
                new_state: &s4,
                direction: Direction::Left,
            },
            Transition {
                current_state: &s4,
                current_char: '1',
                new_char: '1',
                new_state: &s4,
                direction: Direction::Left,
            },
            Transition {
                current_state: &s4,
                current_char: '0',
                new_char: '0',
                new_state: &s5,
                direction: Direction::Left,
            },
            Transition {
                current_state: &s5,
                current_char: '1',
                new_char: '1',
                new_state: &s5,
                direction: Direction::Left,
            },
            Transition {
                current_state: &s5,
                current_char: '0',
                new_char: '1',
                new_state: &s1,
                direction: Direction::Right,
            },
        ];

        let mut turing_machine = TuringMachine::new(
            Tape::new(vec!['1', '0'], vec!['1', '1', '0', '0', '0']),
            transitions,
        );

        turing_machine.run();
        let output: String = turing_machine.tape.input.iter().collect();
        println!("{}", output);
    }
}
