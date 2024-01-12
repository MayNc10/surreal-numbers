use crate::computer::Position;
use itertools::Itertools;
use nannou::color::{Rgba8, Srgb, BLUE, CYAN, PINK, RED};
use petgraph::data::DataMap;
use petgraph::prelude::*;
use rand::distributions::Standard;
use rand::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]

pub enum Color {
    Red,
    Blue,
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0..2) {
            0 => Color::Red,
            1 => Color::Blue,
            _ => unreachable!(),
        }
    }
}

impl Color {
    pub fn get_color(&self) -> Srgb<u8> {
        match self {
            Color::Red => RED,
            Color::Blue => BLUE,
        }
    }

    pub fn get_light_color(&self) -> Rgba8 {
        let mut color: Rgba8 = match self {
            Color::Red => RED,
            Color::Blue => BLUE,
        }
        .into();
        color.alpha = 128;
        color
    }

    pub fn get_border_color(&self) -> Srgb<u8> {
        match self {
            Color::Red => PINK,
            Color::Blue => CYAN,
        }
    }

    pub fn invert(&self) -> Color {
        match self {
            Color::Blue => Color::Red,
            Color::Red => Color::Blue,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    graph: StableUnGraph<(bool, (f32, f32)), Color>,
    turn: Color,
}

impl Game {
    pub fn random_triangles(size: usize, rng: &mut ThreadRng) -> Game {
        let mut current_color: Color = rng.gen();

        let mut points: Vec<_> = [(0f32, 0f32)]
            .into_iter()
            .chain((0..1).map(|i| (i as f32, 1f32)))
            .collect();

        let mut graph = StableUnGraph::with_capacity(size, 2 * size);
        graph.add_node((false, points[0]));
        for (i, &point) in points.iter().enumerate().skip(1) {
            graph.add_node((false, point));
            graph.add_edge(NodeIndex::new(0), NodeIndex::new(i), current_color);
            current_color = current_color.invert();
        }

        let mut free_edges = graph.edge_indices().collect::<Vec<_>>();

        for i in points.len()..size {
            let edge_index = rng.gen_range(0..free_edges.len());
            let edge = free_edges[edge_index];
            free_edges.remove(edge_index);
            let (ai, bi) = graph.edge_endpoints(edge).unwrap();
            let (a, b) = (ai.index(), bi.index());
            let (x1, y1) = points[a];
            let (x2, y2) = points[b];
            const RANDOMNESS: f32 = 0.2;
            let (x, y) = (
                (x1 + x2) / 2.0
                    + (y2 - y1) / 2.0 * (1.0 - RANDOMNESS + RANDOMNESS * 2.0 * rng.gen::<f32>()),
                (y1 + y2) / 2.0
                    + (x1 - x2) / 2.0 * (1.0 - RANDOMNESS + RANDOMNESS * 2.0 * rng.gen::<f32>()),
            );
            points.push((x, y));
            graph.add_node((false, (x, y)));
            graph.add_edge(NodeIndex::new(i), ai, current_color);
            current_color = current_color.invert();
            graph.add_edge(NodeIndex::new(i), bi, current_color);
            current_color = current_color.invert();
            free_edges.push(graph.find_edge(NodeIndex::new(i), ai).unwrap());
            free_edges.push(graph.find_edge(NodeIndex::new(i), bi).unwrap());
        }

        let lowest = points
            .iter()
            .min_by(|(_, y1), (_, y2)| y1.partial_cmp(y2).unwrap())
            .unwrap()
            .1;

        for i in 1..points.len() {
            let (x, y) = points[i];
            points[i] = (x, y - lowest + 0.3);
            *graph.node_weight_mut(NodeIndex::new(i)).unwrap() = (false, points[i]);
        }

        Game {
            graph,
            turn: current_color.invert(),
        }
    }

    pub fn make_move(&self, target: EdgeIndex) -> Game {
        let mut new_state = (*self).clone();
        new_state.turn = new_state.turn.invert();
        new_state.graph.remove_edge(target);
        let base = new_state.graph.node_weight(NodeIndex::new(0)).unwrap().0;
        let mut stack = vec![NodeIndex::new(0)];
        while let Some(node) = stack.pop() {
            for neighbor in new_state.graph.neighbors(node).collect_vec().clone() {
                let tag = new_state
                    .graph
                    .node_weight_mut(NodeIndex::new(neighbor.index()))
                    .unwrap();
                if tag.0 == base {
                    stack.push(neighbor);
                    tag.0 = !base;
                }
            }
        }

        for edge in new_state.graph.edge_indices().collect_vec().clone() {
            let (a, b) = new_state.graph.edge_endpoints(edge).unwrap();
            if new_state.graph.node_weight(a).unwrap().0 == base
                && new_state.graph.node_weight(b).unwrap().0 == base
            {
                new_state.graph.remove_edge(edge);
            }
        }

        for node in new_state.graph.node_indices().collect_vec().clone() {
            if new_state.graph.node_weight(node).unwrap().0 == base {
                new_state.graph.remove_node(node);
            }
        }

        new_state
    }

    pub fn add_branch(&self, target: NodeIndex, position: (f32, f32)) -> Game {
        let mut new_state = (*self).clone();
        let new_node = new_state.graph.add_node((false, position));
        new_state.graph.add_edge(target, new_node, self.turn);
        new_state
    }

    pub fn get_graph(&self) -> &StableUnGraph<(bool, (f32, f32)), Color> {
        &self.graph
    }

    pub fn get_graph_mut(&mut self) -> &mut StableUnGraph<(bool, (f32, f32)), Color> {
        &mut self.graph
    }

    pub fn get_turn(&self) -> Color {
        self.turn
    }

    pub fn switch_turn(&mut self) {
        self.turn = self.turn.invert();
    }
}
