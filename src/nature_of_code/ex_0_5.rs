use crate::nature_of_code::exercise::Exercise;
use nannou::prelude::*;
use nannou_egui::{self, FrameCtx};
use rand::prelude::*;
use rand_distr::{Distribution, Normal};

pub fn init(_app: &App) -> Box<dyn Exercise> {
    Box::new(Model {
        x: 0.,
        y: 0.,
        rand: rand::rng(),
    })
}

struct Model {
    x: f32,
    y: f32,
    rand: ThreadRng,
}

impl Exercise for Model {
    fn update(&mut self, _app: &App, _update: Update, _ui_ctx: &FrameCtx) {
        let normal = Normal::new(0., 1.).unwrap();
        self.x += normal.sample(&mut self.rand);
        self.y += normal.sample(&mut self.rand);
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        draw.rect().x(self.x).y(self.y).w(1.).h(1.).color(BLACK);

        draw.to_frame(app, &frame).unwrap();
    }
}
