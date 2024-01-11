mod computer;
mod hackenbush;

use crate::hackenbush::Color;
use hackenbush::Color::{Blue, Red};
use hackenbush::Game;
use itertools::Itertools;
use nannou::prelude::*;
use petgraph::prelude::*;
use rand::thread_rng;

struct Model {
    _window: window::Id,
    game: Game,
    points: Vec<(f32, f32)>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let win = app.new_window().size(800, 600).view(view).build().unwrap();

    let (game, points) = Game::random_triangles(20, &mut thread_rng());
    Model {
        _window: win,
        game,
        points,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();

    let draw = app.draw();

    draw.background()
        .color(model.game.get_turn().get_border_color());

    let backdrop = win.pad(20.0);
    draw.rect()
        .xy(backdrop.xy())
        .wh(backdrop.wh())
        .color(LIGHTGREY);

    let min_x = model
        .points
        .iter()
        .map(|&(x, _)| x)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_x = model
        .points
        .iter()
        .map(|&(x, _)| x)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_y = 0.0;
    let max_y = model
        .points
        .iter()
        .map(|&(_, y)| y)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let win_max_x = win.w() / 3.0;
    let win_max_y = win.h() / 3.0;
    let win_min_x = -win_max_x;
    let win_min_y = -win_max_y;

    let transform = |(x, y): (f32, f32)| {
        let x = (x - min_x) / (max_x - min_x) * (win_max_x - win_min_x) + win_min_x;
        let y = (y - min_y) / (max_y - min_y) * (win_max_y - win_min_y) + win_min_y;
        (x, y)
    };

    let edges = get_edge_positions(model.game.get_graph(), &model.points, &transform);

    if let Some(edge) =
        get_selected_edge(app.mouse.position().into(), model.game.get_turn(), &edges)
    {
        let (start, end, color) = edges[edge];
        if color == model.game.get_turn() {
            draw.line()
                .start(pt2(start.0, start.1))
                .end(pt2(end.0, end.1))
                .color(color.get_light_color())
                .stroke_weight(10.0);
        }
    }

    for (start, end, color) in edges {
        draw.line()
            .start(start.into())
            .end(end.into())
            .color(color.get_color())
            .stroke_weight(5.0);
    }

    for node in model.game.get_graph().node_indices().skip(1) {
        let (x, y) = transform(model.points[node.index()]);
        draw.ellipse().x_y(x, y).radius(5.0).color(BLACK);
    }

    draw.line()
        .start(pt2(-win.w() / 2.0 + 20.0, win_min_y))
        .end(pt2(win.w() / 2.0 - 20.0, win_min_y))
        .color(BLACK)
        .stroke_weight(5.0);

    draw.to_frame(app, &frame).unwrap();
}

fn get_edge_positions(
    graph: &StableUnGraph<(), Color>,
    points: &[(f32, f32)],
    transform: &impl Fn((f32, f32)) -> (f32, f32),
) -> Vec<((f32, f32), (f32, f32), Color)> {
    graph
        .edge_indices()
        .map(|edge| {
            let (a, b) = graph.edge_endpoints(edge).unwrap();
            let (first, second) = if a.index() == 0 {
                let (x2, y2) = transform(points[b.index()]);
                ((x2, transform((0.0, 0.0)).1), (x2, y2))
            } else if b.index() == 0 {
                let (x1, y1) = transform(points[a.index()]);
                ((x1, transform((0.0, 0.0)).1), (x1, y1))
            } else {
                let (x1, y1) = transform(points[a.index()]);
                let (x2, y2) = transform(points[b.index()]);
                ((x1, y1), (x2, y2))
            };
            (first, second, *graph.edge_weight(edge).unwrap())
        })
        .collect()
}

fn get_selected_edge(
    point: (f32, f32),
    color: Color,
    edges: &[((f32, f32), (f32, f32), Color)],
) -> Option<usize> {
    let distances: Vec<_> = edges
        .iter()
        .enumerate()
        .filter_map(|(i, &(start, end, c))| {
            if c != color {
                return None;
            }
            let length = f32::hypot(start.0 - end.0, start.1 - end.1);
            let distance = if length == 0.0 {
                f32::hypot(start.0 - point.0, start.1 - point.1)
            } else {
                let s_to_p = (point.0 - start.0, point.1 - start.1);
                let s_to_e = (end.0 - start.0, end.1 - start.1);
                let dot = s_to_p.0 * s_to_e.0 + s_to_p.1 * s_to_e.1;
                let t = f32::clamp(dot / (length * length), 0.0, 1.0);
                let closest = (start.0 + t * s_to_e.0, start.1 + t * s_to_e.1);
                f32::hypot(closest.0 - point.0, closest.1 - point.1)
            };
            if distance < 7.0 {
                Some((i, distance))
            } else {
                None
            }
        })
        .sorted_by(|(_, first), (_, second)| first.partial_cmp(second).unwrap())
        .map(|(i, _)| i)
        .collect();

    distances.first().copied()
}
