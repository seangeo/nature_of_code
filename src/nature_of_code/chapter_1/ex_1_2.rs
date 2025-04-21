use crate::nature_of_code::exercise::Exercise;
use nannou::prelude::*;
use nannou_egui::{self, FrameCtx};

pub fn init(app: &App) -> Box<dyn Exercise> {
    Box::new(model(app))
}

struct Model {
    bounds: geom::Rect,
    ball: Ball,
}

struct Ball {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
}

fn model(_app: &App) -> Model {
    Model {
        bounds: Rect::from_wh(vec2(500., 350.)),
        ball: Ball {
            position: Vec2::ZERO,
            velocity: vec2(2., 2.),
            radius: 20.,
        },
    }
}

impl Exercise for Model {
    fn update(&mut self, _app: &App, _update: Update, _ui_ctx: &FrameCtx) {
        let ball_rect = Rect::from_xy_wh(self.ball.position, Vec2::splat(self.ball.radius * 2.));

        if ball_rect.top() >= self.bounds.top() || ball_rect.bottom() <= self.bounds.bottom() {
            self.ball.velocity *= vec2(1., -1.);
        }

        if ball_rect.left() <= self.bounds.left() || ball_rect.right() >= self.bounds.right() {
            self.ball.velocity *= vec2(-1., 1.);
        }

        self.ball.position += self.ball.velocity;
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();
        frame.clear(WHITE);

        draw.ellipse()
            .xy(self.ball.position)
            .radius(self.ball.radius)
            .color(GRAY)
            .stroke_weight(1.)
            .stroke(BLACK);

        draw.rect()
            .xy(self.bounds.xy())
            .wh(self.bounds.wh())
            .no_fill()
            .stroke_weight(2.)
            .stroke(BLACK);

        draw.to_frame(app, &frame).unwrap();
    }
}
