use nannou::prelude::*;

mod domino;
use domino::Domino;

const DOMINO_SHAPE: (f32, f32, f32) = (10.0, 3.0, 20.0);

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    frame: i32,
    dominoes: Vec<Domino>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(600, 600)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    Model {
        frame: 0,
        dominoes: vec![
            Domino::new(vec2(0.0, 0.0), vec2(0.0, 1.0), DOMINO_SHAPE),
            Domino::new(vec2(0.0, 10.0), vec2(0.0, 1.0), DOMINO_SHAPE),
            Domino::new(vec2(0.0, 20.0), vec2(0.0, 1.0), DOMINO_SHAPE),
            Domino::new(vec2(0.0, 30.0), vec2(0.0, 1.0), DOMINO_SHAPE),
        ]
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => cascade(&mut model.dominoes, vec![0]),
        _ => (),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.frame % 10 == 0 {
        for domino in model.dominoes.iter_mut() {
            domino.update();
        }
    }

    model.frame += 1;
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

fn cascade(dominoes: &mut Vec<Domino>, to_knock: Vec<usize>) {
    for d in to_knock {
        dominoes.get_mut(d).unwrap().knock();
    }

    let mut falling = vec![];

    for (i, domino) in dominoes.iter_mut().enumerate() {
        domino.update();
        if domino.falling() {
            falling.push(i);
        }
    }

    for a in 0..dominoes.len() {
        let (fir, sec) = dominoes.split_at_mut(a);
        for b in 0..fir.len() {
            let dom_a = sec.get_mut(0).unwrap();
            let dom_b = fir.get_mut(b).unwrap();

            if falling.iter().any(|&i| i == a) {dom_b.knock();}     // TODO: Figure out why this
                                                                    // isn't working
            if falling.iter().any(|&i| i == b) {dom_a.knock();}
        }
    }
}
