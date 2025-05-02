use crate::nature_of_code::exercise::Exercise;
use nannou::{
    noise::{BasicMulti, NoiseFn},
    prelude::*,
};
use nannou_egui::{self, FrameCtx};

pub fn init(app: &App) -> Box<dyn Exercise> {
    Box::new(model(app))
}

struct Model {
    particles: Vec<Particle>,
}

struct Particle {
    position: Vec2,
    velocity: Vec2,
}

impl Particle {
    fn flow(&mut self) -> &mut Self {
        self.position += self.velocity;
        self
    }

    fn update_velocity(&mut self, direction: f32) -> &mut Self {
        let direction_radians = direction * PI * 2.;
        self.velocity = vec2(direction_radians.cos(), direction_radians.sin()) * 2.;
        self
    }
}

fn model(app: &App) -> Model {
    let window = app.window_rect();
    let width = window.w();
    let height = window.h();

    let particles: Vec<Particle> = (0..1000)
        .map(|_| Particle {
            position: vec2(
                map_range(rand::random(), 0., 1., -width, width),
                map_range(rand::random(), 0., 1., -height, height),
            ),
            velocity: Vec2::ZERO,
        })
        .collect();

    Model { particles }
}

impl Exercise for Model {
    fn update(&mut self, app: &App, _update: Update, _ui_ctx: &FrameCtx) {
        let noise = BasicMulti::new();

        for particle in self.particles.iter_mut() {
            let direction = noise.get([
                particle.position.x as f64 / 150.,
                particle.position.y as f64 / 150.,
                app.time as f64 / 100.,
            ]);
            particle.update_velocity(direction as f32).flow();
        }
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        for particle in &self.particles {
            draw.ellipse()
                .xy(particle.position)
                .color(BLACK)
                .wh(vec2(1.5, 1.5));
        }

        draw.to_frame(app, &frame).unwrap();
    }
}
