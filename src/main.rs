use nannou::prelude::*;

mod domino;
use domino::Domino;

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
        .build()
        .unwrap();
    Model {}
}
