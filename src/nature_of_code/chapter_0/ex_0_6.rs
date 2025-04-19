use crate::nature_of_code::exercise::Exercise;
use nannou::prelude::*;
use nannou_egui::{self, FrameCtx};

pub fn init(_app: &App) -> Box<dyn Exercise> {
    Box::new(Model { x: 0., y: 0. })
}

struct Model {
    x: f32,
    y: f32,
}

impl Exercise for Model {
    fn update(&mut self, _app: &App, _update: Update, _ui_ctx: &FrameCtx) {
        self.x += quadratic_distribution_sample();
        self.y += quadratic_distribution_sample();
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        draw.rect().x(self.x).y(self.y).w(1.).h(1.).color(BLACK);

        draw.to_frame(app, &frame).unwrap();
    }
}

fn quadratic_distribution_sample() -> f32 {
    loop {
        let r1: f32 = map_range(nannou::rand::random(), 0., 1., -2., 2.);
        let probability = r1 * r1;
        let r2: f32 = map_range(nannou::rand::random(), 0., 1., 0., 4.);

        if r2 < probability {
            return r1;
        }
    }
}
