use crate::hackenbush::{Color, Game};
use itertools::Itertools;
use nannou::prelude::*;
use petgraph::graph::EdgeIndex;
use petgraph::prelude::StableUnGraph;
use rand::thread_rng;
use std::collections::HashMap;
use nannou::winit::event::VirtualKeyCode;
use crate::computer::find_best_move;

const SIZE: usize = 4;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ModelMode {
    Playing,
    Building
}

pub struct Model {
    _window: window::Id,
    game: Game,
    transform_data: ((f32, f32), (f32, f32)),
    mode: ModelMode,
}

impl Model {
    fn transform(&self, point: (f32, f32)) -> (f32, f32) {
        let multiplier_x = self.transform_data.0 .0;
        let offset_x = self.transform_data.0 .1;
        let multiplier_y = self.transform_data.1 .0;
        let offset_y = self.transform_data.1 .1;

        let (x, y) = point;
        let x = x * multiplier_x + offset_x;
        let y = y * multiplier_y + offset_y;
        (x, y)
    }

    fn trans_func(&self) -> impl Fn((f32, f32)) -> (f32, f32) + '_ {
        |p: (f32, f32)| self.transform(p)
    }
}

pub fn model(app: &App) -> Model {
    let win = app.new_window().size(800, 600).view(view).build().unwrap();

    let game = Game::random_triangles(SIZE, &mut thread_rng());

    let min_x = game
        .get_graph()
        .node_weights()
        .map(|&(_, (x, _))| x)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_x = game
        .get_graph()
        .node_weights()
        .map(|&(_, (x, _))| x)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_y = 0.0;
    let max_y = game
        .get_graph()
        .node_weights()
        .map(|&(_, (_, y))| y)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let win_max_x = app.window_rect().w() / 3.0;
    let win_max_y = app.window_rect().h() / 3.0;
    let win_min_x = -win_max_x;
    let win_min_y = -win_max_y;

    let multiplier_x = (win_max_x - win_min_x) / (max_x - min_x);
    let offset_x = win_min_x - min_x * multiplier_x;
    let multiplier_y = (win_max_y - win_min_y) / (max_y - min_y);
    let offset_y = win_min_y - min_y * multiplier_y;

    let transform_data = ((multiplier_x, offset_x), (multiplier_y, offset_y));

    Model {
        _window: win,
        game,
        transform_data,
        mode: ModelMode::Playing
    }
}

pub fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            simple: Some(event),
        } => match event {
            MousePressed(MouseButton::Left) => {
                if model.mode == ModelMode::Playing {
                    let edges = get_edge_positions(model.game.get_graph(), &model.trans_func());
                    let closest_edge =
                        get_selected_edge(app.mouse.position().into(), model.game.get_turn(), &edges);

                    if let Some(edge) = closest_edge {
                        model.game = model.game.make_move(edge);
                    }
                }
                else if model.mode == ModelMode::Building {

                }
            }
            KeyPressed(VirtualKeyCode::Return) => {
                let value = find_best_move(&model.game).score;
                println!("Got surreal value");
                println!("Model evaluation is: {}", value.to_real())
            }
            _ => {}
        },
        _ => {}
    };
}

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

    let edges = get_edge_positions(model.game.get_graph(), &model.trans_func());

    if let Some(edge) =
        get_selected_edge(app.mouse.position().into(), model.game.get_turn(), &edges)
    {
        let (start, end, color) = edges[&edge];
        if color == model.game.get_turn() {
            draw.line()
                .start(pt2(start.0, start.1))
                .end(pt2(end.0, end.1))
                .color(color.get_light_color())
                .stroke_weight(10.0);
        }
    }

    for (_, &(start, end, color)) in edges.iter().sorted_unstable_by_key(|(_, (_, _, c))| *c) {
        draw.line()
            .start(start.into())
            .end(end.into())
            .color(color.get_color())
            .stroke_weight(5.0);
    }

    for node in model.game.get_graph().node_indices().skip(1) {
        let (x, y) = model.transform(model.game.get_graph().node_weight(node).unwrap().1);
        draw.ellipse().x_y(x, y).radius(5.0).color(BLACK);
    }

    draw.line()
        .start(pt2(-win.w() / 2.0 + 20.0, model.transform((0.0, 0.0)).1))
        .end(pt2(win.w() / 2.0 - 20.0, model.transform((0.0, 0.0)).1))
        .color(BLACK)
        .stroke_weight(5.0);

    draw.to_frame(app, &frame).unwrap();
}

fn get_edge_positions(
    graph: &StableUnGraph<(bool, (f32, f32)), Color>,
    transform: &impl Fn((f32, f32)) -> (f32, f32),
) -> HashMap<EdgeIndex, ((f32, f32), (f32, f32), Color)> {
    graph
        .edge_indices()
        .map(|edge| {
            let (a, b) = graph.edge_endpoints(edge).unwrap();
            let (first, second) = if a.index() == 0 {
                let (x2, y2) = transform(graph.node_weight(b).unwrap().1);
                ((x2, transform((0.0, 0.0)).1), (x2, y2))
            } else if b.index() == 0 {
                let (x1, y1) = transform(graph.node_weight(a).unwrap().1);
                ((x1, transform((0.0, 0.0)).1), (x1, y1))
            } else {
                let (x1, y1) = transform(graph.node_weight(a).unwrap().1);
                let (x2, y2) = transform(graph.node_weight(b).unwrap().1);
                ((x1, y1), (x2, y2))
            };
            (edge, (first, second, *graph.edge_weight(edge).unwrap()))
        })
        .collect()
}

fn get_selected_edge(
    point: (f32, f32),
    color: Color,
    edges: &HashMap<EdgeIndex, ((f32, f32), (f32, f32), Color)>,
) -> Option<EdgeIndex> {
    let distances: Vec<_> = edges
        .iter()
        .filter_map(|(&i, &(start, end, c))| {
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
