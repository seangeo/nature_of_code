use crate::nature_of_code::exercise::Exercise;
use nannou::prelude::*;
use nannou_egui::{self, FrameCtx};

pub fn init(app: &App) -> Box<dyn Exercise> {
    Box::new(model(app))
}

struct Model {
    position: Vec2,
}

fn model(_app: &App) -> Model {
    Model {
        position: vec2(0., 0.),
    }
}

impl Exercise for Model {
    fn update(&mut self, app: &App, _update: Update, _ui_ctx: &FrameCtx) {
        let step: f32 = nannou::rand::random();

        if step >= 0.5 {
            let mouse_x = app.mouse.x;
            let mouse_y = app.mouse.y;
            if mouse_x != 0. {
                let mouse_vector = vec2(mouse_x, mouse_y);
                let direction = (mouse_vector - self.position).normalize();

                self.position += direction;
            }
        } else if step >= 0.4375 {
            self.position += vec2(-1., -1.);
        } else if step >= 0.375 {
            self.position += vec2(-1., 0.);
        } else if step >= 0.3125 {
            self.position += vec2(-1., 1.);
        } else if step >= 0.25 {
            self.position += vec2(0., 1.);
        } else if step >= 0.1875 {
            self.position += vec2(1., 1.);
        } else if step >= 0.125 {
            self.position += vec2(1., 0.);
        } else if step >= 0.0625 {
            self.position += vec2(1., -1.);
        } else {
            self.position += vec2(0., -1.);
        }
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        draw.rect().xy(self.position).w(1.).h(1.).color(BLACK);

        draw.to_frame(app, &frame).unwrap();
    }
}
