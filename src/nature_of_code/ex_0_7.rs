use crate::nature_of_code::exercise::Exercise;
use nannou::noise::NoiseFn;
use nannou::prelude::*;
use nannou_egui::{self, FrameCtx};

pub fn init(_app: &App) -> Box<dyn Exercise> {
    Box::new(Model {
        x: 0.,
        y: 0.,
        tx: 0.,
        ty: 100000.,
        noise: nannou::noise::Perlin::new(),
    })
}

struct Model {
    x: f32,
    y: f32,
    tx: f64,
    ty: f64,
    noise: nannou::noise::Perlin,
}

impl Exercise for Model {
    fn update(&mut self, _app: &App, _update: Update, _ui_ctx: &FrameCtx) {
        self.tx += 0.05;
        self.ty += 0.05;

        self.x += self.noise.get([self.tx, self.ty]) as f32;
        self.y += self.noise.get([self.ty, self.tx]) as f32;
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        draw.rect().x(self.x).y(self.y).w(1.).h(1.).color(BLACK);

        draw.to_frame(app, &frame).unwrap();
    }
}
