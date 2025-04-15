use crate::nature_of_code::exercise::Exercise;
use nannou::prelude::*;
use nannou_egui::{self, FrameCtx};

pub fn init(app: &App) -> Box<dyn Exercise> {
    Box::new(model(app))
}

struct Model {
    x: f32,
    y: f32,
}

fn model(_app: &App) -> Model {
    Model { x: 0., y: 0. }
}

impl Exercise for Model {
    fn update(&mut self, app: &App, _update: Update, _ui_ctx: &FrameCtx) {
        let step: f32 = nannou::rand::random();

        if step >= 0.5 {
            let mouse_x = app.mouse.x;
            let mouse_y = app.mouse.y;
            if mouse_x != 0. {
                let mouse_vector = vec2(mouse_x, mouse_y);
                let self_vector = vec2(self.x, self.y);
                let direction = (mouse_vector - self_vector).normalize();

                self.x += direction.x;
                self.y += direction.y;
            }
        } else if step >= 0.4375 {
            self.x -= 1.;
            self.y -= 1.;
        } else if step >= 0.375 {
            self.x -= 1.;
        } else if step >= 0.3125 {
            self.x -= 1.;
            self.y += 1.;
        } else if step >= 0.25 {
            self.y += 1.;
        } else if step >= 0.1875 {
            self.x += 1.;
            self.y += 1.;
        } else if step >= 0.125 {
            self.x += 1.;
        } else if step >= 0.0625 {
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
