mod app;
mod computer;
mod hackenbush;
mod surreals;

use nannou::prelude::*;
use crate::surreals::SURREALS;

fn main() {
    nannou::app(app::model).event(app::event).run();
}
