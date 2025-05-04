use std::collections::VecDeque;

use crate::nature_of_code::{exercise::Exercise, noise_config::NoiseConfig};
use nannou::{noise::NoiseFn, prelude::*};
use nannou_egui::{self, egui, FrameCtx};

pub fn init(app: &App) -> Box<dyn Exercise> {
    Box::new(Model::new(app))
}

struct FlowLine {
    position: Vec2,
    velocity: Vec2,
    tail: VecDeque<Vec2>,
    trail_length: usize,
}

impl FlowLine {
    fn new(width: f32, height: f32, trail_length: usize) -> Self {
        FlowLine {
            position: vec2(
                map_range(rand::random(), 0., 1., -width, width),
                map_range(rand::random(), 0., 1., -height, height),
            ),
            velocity: Vec2::ZERO,
            tail: VecDeque::with_capacity(trail_length),
            trail_length,
        }
    }

    fn flow(&mut self) -> &mut Self {
        if self.tail.len() >= self.trail_length {
            self.tail.pop_front();
        }
        self.tail.push_back(self.position);

        self.position += self.velocity;
        self
    }

    fn update_velocity(&mut self, direction: f32) -> &mut Self {
        let direction_radians = direction * PI * 2.;
        self.velocity = vec2(direction_radians.cos(), direction_radians.sin()) * 2.;
        self
    }
}

struct Model {
    flow_lines: Vec<FlowLine>,
    noise_config: NoiseConfig,
    clear: bool,
    trail_count: usize,
    trail_length: usize,
}

impl Model {
    fn new(app: &App) -> Model {
        let trail_length = 100;
        let trail_count = 100;

        Model {
            flow_lines: Self::generate_flow_lines(trail_count, app.window_rect(), trail_length),
            noise_config: NoiseConfig::default(),
            clear: false,
            trail_count,
            trail_length,
        }
    }

    fn generate_flow_lines(trail_count: usize, window: Rect, trail_length: usize) -> Vec<FlowLine> {
        (0..=trail_count)
            .map(|_| FlowLine::new(window.w(), window.h(), trail_length))
            .collect()
    }

    fn reset_flow_lines(&mut self, window: Rect) {
        self.flow_lines = Self::generate_flow_lines(self.trail_count, window, self.trail_length);
    }

    fn update_flow_lines(&mut self, time: f64) -> &Self {
        let noise = self.noise_config.create_noise();

        for flow_line in self.flow_lines.iter_mut() {
            let direction = noise.get([
                flow_line.position.x as f64 / self.noise_config.x_fact,
                flow_line.position.y as f64 / self.noise_config.y_fact,
                time / 100.,
            ]);
            flow_line.update_velocity(direction as f32).flow();
        }

        self
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
                        ui.add(
                            egui::Slider::new(&mut self.trail_length, 0..=1000)
                                .text("Trail length"),
                        )
                        .drag_released()
                        .then(|| self.clear = true);

                        ui.add(
                            egui::Slider::new(&mut self.trail_count, 0..=4000).text("Trail count"),
                        )
                        .drag_released()
                        .then(|| self.clear = true);
                    },
                );

                ui.separator();

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
            self.reset_flow_lines(app.window_rect());
        }

        self.update_flow_lines(app.time as f64);
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        draw.background().color(WHITE);

        for flow_line in &self.flow_lines {
            draw.polyline().color(BLACK).points(flow_line.tail.clone());
        }

        draw.to_frame(app, &frame).unwrap();
    }
}
