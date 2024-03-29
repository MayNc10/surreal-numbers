use crate::app::ModelMode::{Building, Playing};
use crate::computer::find_best_move;
use crate::hackenbush::{Color, Game};
use itertools::Itertools;
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;
use petgraph::data::DataMap;
use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::prelude::StableUnGraph;
use rand::thread_rng;
use std::collections::HashMap;

const SIZE: usize = 4;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ModelMode {
    Playing,
    Building,
}

pub struct Model {
    _window: window::Id,
    game: Game,
    transform_data: ((f32, f32), (f32, f32)),
    mode: ModelMode,
    selected_node: Option<NodeIndex>,
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

    fn inverse_transform(&self, point: (f32, f32)) -> (f32, f32) {
        let multiplier_x = self.transform_data.0 .0;
        let offset_x = self.transform_data.0 .1;
        let multiplier_y = self.transform_data.1 .0;
        let offset_y = self.transform_data.1 .1;

        let (x, y) = point;
        let x = (x - offset_x) / multiplier_x;
        let y = (y - offset_y) / multiplier_y;
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
        mode: ModelMode::Playing,
        selected_node: None,
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
                    let (x, y) = app.mouse.position().into();
                    if !app.window_rect().pad(20.0).contains(pt2(x, y))
                        && app.window_rect().contains(pt2(x, y))
                    {
                        model.game.switch_turn();
                    }
                    let edges = get_edge_positions(model.game.get_graph(), &model.trans_func());
                    let closest_edge = get_selected_edge(
                        app.mouse.position().into(),
                        model.game.get_turn(),
                        &edges,
                    );

                    if let Some(edge) = closest_edge {
                        model.game = model.game.make_move(edge);
                    }
                } else if model.mode == Building {
                    let (x, y) = app.mouse.position().into();
                    if !app.window_rect().pad(20.0).contains(pt2(x, y))
                        && app.window_rect().contains(pt2(x, y))
                    {
                        model.game.switch_turn();
                    } else if model.selected_node.is_some() {
                        let nodes = get_node_positions(model.game.get_graph(), &model.trans_func());

                        let new_node;
                        if let Some(node) = get_selected_node(app.mouse.position().into(), &nodes) {
                            new_node = node;
                        } else {
                            let new_node_pos = model.inverse_transform((x, y));
                            let node_weight = (
                                model
                                    .game
                                    .get_graph()
                                    .node_weight(NodeIndex::new(0))
                                    .unwrap()
                                    .0,
                                new_node_pos,
                            );
                            new_node = model.game.get_graph_mut().add_node(node_weight);
                        }
                        let turn = model.game.get_turn();
                        model.game.get_graph_mut().add_edge(
                            model.selected_node.unwrap(),
                            new_node,
                            turn,
                        );
                        model.selected_node = None;
                    } else {
                        // Get node they wanted to click on
                        let nodes = get_node_positions(model.game.get_graph(), &model.trans_func());
                        let closest_node = get_selected_node(app.mouse.position().into(), &nodes);
                        if let Some(node) = closest_node {
                            model.selected_node = Some(node);
                        }
                    }
                }
            }
            KeyPressed(VirtualKeyCode::Return) => {
                let value = find_best_move(&model.game).score;
                println!("Got surreal value");
                println!("Model evaluation is: {}", value.to_real())
            }
            KeyPressed(VirtualKeyCode::M) => {
                model.mode = match model.mode {
                    ModelMode::Building => ModelMode::Playing,
                    ModelMode::Playing => ModelMode::Building,
                };
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

    if model.mode == Building {
        let backdrop = win.pad(10.0);
        draw.rect().xy(backdrop.xy()).wh(backdrop.wh()).color(BLACK);
    }

    let backdrop = win.pad(20.0);
    draw.rect()
        .xy(backdrop.xy())
        .wh(backdrop.wh())
        .color(LIGHTGREY);

    let edges = get_edge_positions(model.game.get_graph(), &model.trans_func());

    if model.mode == Playing {
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
    }

    for (_, &(start, end, color)) in edges.iter().sorted_unstable_by_key(|(_, (_, _, c))| *c) {
        draw.line()
            .start(start.into())
            .end(end.into())
            .color(color.get_color())
            .stroke_weight(5.0);
    }

    if model.mode == Building {
        if let Some(node) = model.selected_node {
            let (x, y) = model.transform(model.game.get_graph().node_weight(node).unwrap().1);
            draw.ellipse().x_y(x, y).radius(10.0).color(GRAY);
        }

        let nodes = get_node_positions(model.game.get_graph(), &model.trans_func());
        if let Some(node) = get_selected_node(app.mouse.position().into(), &nodes) {
            let (x, y) = model.transform(model.game.get_graph().node_weight(node).unwrap().1);
            draw.ellipse().x_y(x, y).radius(10.0).color(GRAY);
        }
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

fn get_node_positions(
    graph: &StableUnGraph<(bool, (f32, f32)), Color>,
    transform: &impl Fn((f32, f32)) -> (f32, f32),
) -> HashMap<NodeIndex, (f32, f32)> {
    graph
        .node_indices()
        .map(|node| (node, transform(graph.node_weight(node).unwrap().1)))
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

fn get_selected_node(
    point: (f32, f32),
    nodes: &HashMap<NodeIndex, (f32, f32)>,
) -> Option<NodeIndex> {
    nodes
        .iter()
        .map(|(i, (nx, ny))| (i, f32::hypot(nx - point.0, ny - point.1)))
        .filter(|(_, dist)| *dist <= 7.0f32)
        .sorted_unstable_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
        .map(|(i, _)| *i)
        .next()
}
