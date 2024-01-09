use petgraph::prelude::*;

use crate::app;

pub enum Color {
    Red,
    Blue,
}

pub struct Game {
    graph: UnGraph<app::Coordinate, Color>,
    turn: Color,
}

impl Game {
    pub fn make_move(&self, _move: usize) -> Game {
        panic!("TODO")
    }
}