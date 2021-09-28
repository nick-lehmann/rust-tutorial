#![allow(dead_code)]

#[derive(PartialEq, Eq)]
pub enum StateType {
    Start,
    Final,
    None,
}

#[derive(PartialEq, Eq)]
pub struct State {
    pub name: String,
    pub state_type: StateType,
}

impl State {
    pub fn new(name: String, state_type: StateType) -> Self {
        State { name, state_type }
    }
}
