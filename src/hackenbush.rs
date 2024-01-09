use petgraph::prelude::*;

pub enum Color {
    Red,
    Blue,
}

pub struct Game {
    graph: UnGraph<(), Color>,
    turn: Color,
}

impl Game {
    pub fn make_move(&self, _move: usize) -> Game {
        panic!("TODO")
    }
}