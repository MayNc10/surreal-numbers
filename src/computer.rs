use petgraph::graph::EdgeIndex;
use crate::hackenbush::{Color, Game};
use petgraph::visit::{EdgeRef, IntoEdgeReferences};
use crate::surreals::{Surreal, SurrealNumbers, SURREALS};
use rayon::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Position {
    pub score: Surreal,
    pub best_move: Option<usize>,
}

pub fn find_best_move(game: &Game) -> Position {
    // We can optimize this later
    println!("Finding best move!");
    find_best_move_subgraph(game, game.get_turn())
}

fn find_best_move_subgraph(game: &Game, player: Color) -> Position {
    let graph = game.get_graph();
    //println!("Graph has {} edges", graph.edge_count());


    let mut blue_values = Vec::new();
    let mut red_values = Vec::new();

    let positions: Vec<(Position, EdgeIndex, &Color)> =
        graph.edge_references()
        .into_iter()
        .map(|edge_ref| (edge_ref.weight(), edge_ref.id()))
        .collect::<Vec<_>>()
        .into_iter()
        .map(|(edge, index)| {
            let new_game = game.make_move(index);
            let game_value = //Position {score: unsafe { Surreal::new_with_number_collection(None, None, &mut *unlocked_surreals) }, best_move: None};
                find_best_move_subgraph(&new_game, edge.invert());
            (game_value, index, edge)
        })
        .collect::<Vec<_>>();


    for (game_value, index, edge) in positions {
        match edge {
            Color::Blue => &mut blue_values,
            Color::Red => &mut red_values,
        }
            .push((game_value, index))
    }


    // Now calculate the simplified value of the position
    let left_hand = blue_values.iter().max().copied();
    let right_hand = red_values.iter().min().copied();


    let surreal_value = Surreal::new( left_hand.map(|(p, _)| p.score), right_hand.map(|(p, _)| p.score));
    let best_move = match player {
        Color::Blue => left_hand.map(|(_, m)| m.index()),
        Color::Red => right_hand.map(|(_, m)| m.index()),
    };

    Position {score: surreal_value, best_move}
}
