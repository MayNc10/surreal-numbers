use petgraph::prelude::*;

use crate::app;

pub enum Color {
    Red,
    Blue,
}

pub type Graph = UnGraph<app::Coordinate, Color>;

pub struct Game {
    graph: Graph,
    turn: Color,
}

impl Game {
    pub fn make_move(&self, _move: usize) -> Game {
        panic!("TODO")
    }
}