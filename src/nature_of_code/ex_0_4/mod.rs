use crate::nature_of_code::exercise::Exercise;
use nannou::prelude::*;
use nannou_egui::{self, FrameCtx, egui};
use rand_distr::{Distribution, Normal};

pub fn init(app: &App) -> Box<dyn Exercise> {
    Box::new(model(app))
}

struct Model {
    splatter_stddev: f32,
    color_stddev: f32,
}

fn model(_app: &App) -> Model {
    Model {
        splatter_stddev: 80.,
        color_stddev: 0.2,
    }
}

impl Exercise for Model {
    fn update(&mut self, _app: &App, _update: Update, ctx: &FrameCtx) {
        egui::TopBottomPanel::bottom("Settings").show(&ctx, |ui| {
            ui.add(
                egui::Slider::new(&mut self.splatter_stddev, 0.0..=200.0).text("Splatter Std Dev."),
            );
            ui.add(egui::Slider::new(&mut self.color_stddev, 0.0..=1.0).text("Colour Std Dev."));
        });
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        if frame.nth() == 0 {
            draw.background().color(WHITE);
        }

        draw_dot(&draw, self.splatter_stddev, self.color_stddev);

        draw.to_frame(app, &frame).unwrap();
    }
}

fn draw_dot(draw: &Draw, splatter_stddev: f32, color_stddev: f32) {
    let mut r = rand::rng();
    let normal_pos = Normal::new(0., splatter_stddev).unwrap();
    let vx = normal_pos.sample(&mut r);
    let vy = normal_pos.sample(&mut r);

    let normal_col = Normal::new(0.5, color_stddev).unwrap();
    let mut r2 = rand::rng();
    let r = normal_col.sample(&mut r2).clamp(0., 1.);
    let g = normal_col.sample(&mut r2).clamp(0., 1.);
    let b = normal_col.sample(&mut r2).clamp(0., 1.);
    let a = normal_col.sample(&mut r2).clamp(0., 1.);

    draw.ellipse().radius(5.0).rgba(r, g, b, a).x(vx).y(vy);
}
