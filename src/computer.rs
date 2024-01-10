use crate::hackenbush::{Color, Game, Graph};

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

fn find_best_move_subgraph(graph: &Graph, player: Color) -> Position {
    // Brute force move search
    // For every move, make that move, then compute the value of the position
    // a position where red loses / blue wins = +1, red wins / blue loses = -1

    // This is bad, but tells us if there's only one edge in the graph (i.e., one move)

    if let Some(g_move) = graph.edge_weights().next() && graph.edge_weights().skip(1).next().is_none() {
        return Position {
            score: match g_move { Color::Blue => ONE_BLUE, Color::Red => ONE_RED },
            best_move: if g_move == player {
                Some( graph.edge_indices().next().unwrap().index() )
            } else { None }
        }
    }

    for edge in graph.edge_weights() {
        if edge == player {

        }
    }

    None
}