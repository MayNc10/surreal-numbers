use crate::hackenbush::{Color, Game};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Position {
    pub score: i32,
    pub best_move: Option<usize>,
}

pub fn find_best_move(_game: &Game, _player: Color) -> Position {
    panic!("TODO")
}

// + -> blue
// - -> red

const ONE_BLUE: i32 = 1;
const ONE_RED: i32 = -1;

fn find_best_move_subgraph(game: &Game, player: Color) -> Position {
    // Brute force move search
    // For every move, make that move, then compute the value of the position
    // a position where red loses / blue wins = +1, red wins / blue loses = -1

    // This is bad, but tells us if there's only one edge in the graph (i.e., one move)

    let graph = game.get_graph();

    let mut blue_values = Vec::new();
    let mut red_values = Vec::new();

    for edge_ref in graph.edge_references() {
        let edge = edge_ref.weight();
        let index = edge_ref.id();

        let new_game = game.make_move(index.index());
        let game_value = find_best_move_subgraph(&new_game, edge.invert());
        match edge {
            Color::Blue => &mut blue_values,
            Color::Red => &mut red_values,
        }
        .push((game_value, index))
    }
    // Now calculate the simplified value of the position
    let left_hand = blue_values.iter().max().copied();
    let right_hand = red_values.iter().max().copied();

    // This is where we need a surreal number system to compute the actual value
    // Then we could return the best move and value

    panic!()
}
