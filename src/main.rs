use nannou::prelude::*;

mod domino;
use domino::{Domino, DominoState};

const DOMINO_SHAPE: (f32, f32, f32) = (10.0, 3.0, 20.0);

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    dominoes: Vec<Domino>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    Model {
        dominoes: vec![
            Domino::new(vec2(0.0, 0.0), vec2(0.0, 1.0), DOMINO_SHAPE),
        ]
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.dominoes.get_mut(0).unwrap().knock(),
        _ => (),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for domino in model.dominoes.iter_mut() {
        domino.update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let _win = app.window_rect();

    draw.background().color(BLACK);

    for domino in model.dominoes.iter() {
        domino.show(&draw);
    }

    draw.to_frame(&app, &frame).unwrap();
}
