use nannou::prelude::*;

mod domino;
use domino::{Domino, DominoState};

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

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let _win = app.window_rect();

    draw.background().color(RED);

    draw.to_frame(&app, &frame).unwrap();
}
