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
        let step: f32 = nannou::rand::random();

        if step >= 0.7 {
            self.x -= 1.;
            self.y -= 1.;
        } else if step >= 0.6 {
            self.x -= 1.;
        } else if step >= 0.5 {
            self.x -= 1.;
            self.y += 1.;
        } else if step >= 0.4 {
            self.y += 1.;
        } else if step >= 0.3 {
            self.x += 1.;
            self.y += 1.;
        } else if step >= 0.2 {
            self.x += 1.;
        } else if step >= 0.1 {
            self.x += 1.;
            self.y -= 1.;
        } else {
            self.y -= 1.;
        }
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        draw.rect().x(self.x).y(self.y).w(1.).h(1.).color(BLACK);

        draw.to_frame(app, &frame).unwrap();
    }
}
