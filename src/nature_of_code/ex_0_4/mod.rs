use nannou::prelude::*;
use rand_distr::{Normal, Distribution};

pub fn run() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
}

fn model(_app: &App) -> Model {
    Model { }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }

    draw_dot(&draw);
    

    draw.to_frame(app, &frame).unwrap();
}

fn draw_dot(draw: &Draw) {
    let mut r = rand::rng();
    let normal_pos = Normal::new(0., 80.).unwrap();
    let vx = normal_pos.sample(&mut r);
    let vy = normal_pos.sample(&mut r);

    let normal_col = Normal::new(0.5, 0.2).unwrap();
    let mut r2 = rand::rng();
    let r = normal_col.sample(&mut r2).clamp(0., 1.);
    let g = normal_col.sample(&mut r2).clamp(0., 1.);
    let b = normal_col.sample(&mut r2).clamp(0., 1.);
    let a = normal_col.sample(&mut r2).clamp(0., 1.);

    draw.ellipse().radius(5.0).rgba(r, g, b, a).x(vx).y(vy);
}

