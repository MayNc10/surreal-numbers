mod hackenbush;
mod computer;

use nannou::prelude::*;
use petgraph::prelude::*;
use hackenbush::Game;

struct Model {
    _window: window::Id,
    // game: Game
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let win = app.new_window()
        .size(800, 600)
        .view(view)
        .build()
        .unwrap();

    Model {
        _window: win,
        // game: Game::new()
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let win = app.window_rect();

    let draw = app.draw();

    draw.background().color(LIGHTGREY);
    draw.line()
        .start(pt2(-win.w() / 2.0, -win.h() / 3.0))
        .end(pt2(win.w() / 2.0, -win.h() / 3.0))
        .color(BLACK)
        .stroke_weight(5.0);

    draw.to_frame(app, &frame).unwrap();
}
