mod app;
mod computer;
mod hackenbush;

use nannou::prelude::*;

fn main() {
    nannou::app(app::model).event(app::event).run();
}
