use nannou::color::{Rgba8, Srgb, BLUE, CYAN, PINK, RED};
use petgraph::prelude::*;
use rand::distributions::Standard;
use rand::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]

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

pub struct Game {
    graph: StableUnGraph<(), Color>,
    turn: Color,
}

impl Game {
    pub fn random_triangles(size: usize, rng: &mut ThreadRng) -> (Game, Vec<(f32, f32)>) {
        let mut points: Vec<_> = [(0f32, 0f32)]
            .into_iter()
            .chain((0..1).map(|i| (i as f32, 1f32)))
            .collect();

        let mut graph = StableUnGraph::with_capacity(size, 2 * size);
        graph.add_node(());
        for (i, _) in points.iter().enumerate().skip(1) {
            graph.add_node(());
            graph.add_edge(NodeIndex::new(0), NodeIndex::new(i), rng.gen());
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
            graph.add_node(());
            graph.add_edge(NodeIndex::new(i), ai, rng.gen());
            graph.add_edge(NodeIndex::new(i), bi, rng.gen());
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
        }

        (
            Game {
                graph,
                turn: rng.gen(),
            },
            points,
        )
    }

    pub fn make_move(&self, _move: usize) -> Game {
        let mut new_state = (*self).clone();
        new_state.turn = new_state.turn.invert();
        new_state
    }

    pub fn get_graph(&self) -> &StableUnGraph<(), Color> {
        &self.graph
    }

    pub fn get_turn(&self) -> Color {
        self.turn
    }
}
