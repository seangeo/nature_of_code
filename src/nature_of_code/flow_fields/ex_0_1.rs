use crate::nature_of_code::{exercise::Exercise, noise_config::NoiseConfig};
use nannou::{noise::NoiseFn, prelude::*};
use nannou_egui::{self, egui, FrameCtx};

pub fn init(app: &App) -> Box<dyn Exercise> {
    Box::new(model(app))
}

struct Model {
    flow_lines: Vec<FlowLine>,
    noise_config: NoiseConfig,
    clear: bool,
}

impl Model {
    fn update_flow_lines(&mut self, time: f64) -> &Self {
        let noise = self.noise_config.create_noise();

        for flow_line in self.flow_lines.iter_mut() {
            let direction = noise.get([
                flow_line.position.x as f64 / 150.,
                flow_line.position.y as f64 / 150.,
                time / 100.,
            ]);
            flow_line.update_velocity(direction as f32).flow();
        }

        self
    }
}

struct FlowLine {
    position: Vec2,
    velocity: Vec2,
}

impl FlowLine {
    fn new(width: f32, height: f32) -> Self {
        FlowLine {
            position: vec2(
                map_range(rand::random(), 0., 1., -width, width),
                map_range(rand::random(), 0., 1., -height, height),
            ),
            velocity: Vec2::ZERO,
        }
    }

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

fn generate_flow_lines(width: f32, height: f32) -> Vec<FlowLine> {
    (0..5000).map(|_| FlowLine::new(width, height)).collect()
}

fn model(app: &App) -> Model {
    let window = app.window_rect();
    let width = window.w();
    let height = window.h();

    Model {
        flow_lines: generate_flow_lines(width, height),
        noise_config: NoiseConfig::default(),
        clear: false,
    }
}

impl Exercise for Model {
    fn update(&mut self, app: &App, _update: Update, ui_ctx: &FrameCtx) {
        if self.clear {
            self.clear = false;
        }

        egui::TopBottomPanel::bottom("Settings").show(&ui_ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(0., 0.),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        if self.noise_config.ui(ui) {
                            self.clear = true;
                        }
                    },
                );
            })
        });

        if self.clear {
            self.flow_lines = generate_flow_lines(app.window_rect().w(), app.window_rect().h());
        }

        self.update_flow_lines(app.time as f64);
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        if self.clear {
            draw.background().color(WHITE);
        }

        for flow_line in &self.flow_lines {
            draw.line()
                .start(flow_line.position - flow_line.velocity)
                .end(flow_line.position)
                .color(BLACK);
        }

        draw.to_frame(app, &frame).unwrap();
    }
}
