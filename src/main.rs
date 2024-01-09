mod hackenbush;
mod computer;
mod app;

use nannou::prelude::*;
use petgraph::prelude::*;
use hackenbush::Game;
use app::*;



fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

