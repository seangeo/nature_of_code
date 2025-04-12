use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    x: f32,
    y: f32,
}

fn model(_app: &App) -> Model {
    Model { x: 0., y: 0. }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let step: f32 = nannou::rand::random();

    if step >= 0.7 {
        model.x -= 1.;
        model.y -= 1.;
    } else if step >= 0.6 {
        model.x -= 1.;
    } else if step >= 0.5 {
        model.x -= 1.;
        model.y += 1.;
    } else if step >= 0.4 {
        model.y += 1.;
    } else if step >= 0.3 {
        model.x += 1.;
        model.y += 1.;
    } else if step >= 0.2 {
        model.x += 1.;
    } else if step >= 0.1 {
        model.x += 1.;
        model.y -= 1.;
    } else {
        model.y -= 1.;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 {

    draw.background().color(WHITE);
    }

    draw.rect()
        .x(model.x)
        .y(model.y)
        .w(1.)
        .h(1.)
        .color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
