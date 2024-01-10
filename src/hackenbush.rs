use petgraph::prelude::*;

use crate::app;

pub enum Color {
    Red,
    Blue,
}

impl Color {
    pub fn invert(&self) -> Color {
        match self {
            Color::Blue => Color::Red,
            Color::Red => Color::Blue,
        }
    }
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
    pub fn graph(&self) -> &Graph { &self.graph }
}