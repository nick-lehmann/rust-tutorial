use crate::direction::Direction;

use std::fmt::{Display, Formatter, Result};

pub struct Tape {
    pub alphabet: Vec<char>,
    pub head: usize,
    pub input: Vec<char>,
}

impl Tape {
    pub fn new(alphabet: Vec<char>, input: Vec<char>) -> Self {
        Tape {
            alphabet,
            input,
            head: 0,
        }
    }

    pub fn read(&self) -> char {
        if self.head > self.input.len() {
            panic!("You're trying to read a character outside the tape")
        }

        self.input[self.head]
    }

    pub fn write(&mut self, character: char) -> () {
        if !(self.alphabet.contains(&character)) {
            return;
        }

        self.input[self.head] = character;
    }

    #[allow(unused_comparisons)]
    pub fn move_head(&mut self, direction: &Direction) -> () {
        match direction {
            Direction::Left => self.head -= 1,
            Direction::Right => self.head += 1,
            Direction::None => {}
        }

        if self.head < 0 {
            self.head = 0
        }

        if self.head >= self.input.len() {
            self.input.push('0')
        }
    }
}

impl Display for Tape {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.input.iter().collect::<String>())
    }
}

pub fn get_lowercase_alphabet() -> Vec<char> {
    (b'a'..b'z').map(|c| c as char).collect()
}
