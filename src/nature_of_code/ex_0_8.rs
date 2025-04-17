use std::time::Instant;

use crate::nature_of_code::exercise::Exercise;
use nannou::image::DynamicImage;
use nannou::prelude::*;
use nannou::wgpu::Texture;
use nannou::{image::ImageBuffer, noise::NoiseFn};
use nannou_egui::{self, egui, FrameCtx};

pub fn init(_app: &App) -> Box<dyn Exercise> {
    let image = ImageBuffer::new(0, 0);

    Box::new(Model {
        image: DynamicImage::ImageRgba8(image),
        build_image: true,
        noise: nannou::noise::BasicMulti::new(),
        x_fact: 50.,
        y_fact: 50.,
    })
}

struct Model {
    image: DynamicImage,
    build_image: bool,
    noise: nannou::noise::BasicMulti,
    x_fact: f64,
    y_fact: f64,
}

impl Model {
    fn pixel_for(&self, x: u32, y: u32) -> nannou::image::Rgba<u8> {
        let n = self
            .noise
            .get([x as f64 / self.x_fact as f64, y as f64 / self.y_fact as f64]);
        let c = map_range(n, -1., 1., 0, 255) as u8;
        nannou::image::Rgba([c, c, c, 100])
    }
}

impl Exercise for Model {
    fn update(&mut self, app: &App, _update: Update, ui_ctx: &FrameCtx) {
        egui::TopBottomPanel::bottom("Settings").show(&ui_ctx, |ui| {
            if ui
                .add(egui::Slider::new(&mut self.x_fact, 0.0..=200.0).text("x-factor"))
                .drag_released()
            {
                self.build_image = true;
            }
            if ui
                .add(egui::Slider::new(&mut self.y_fact, 0.0..=200.0).text("y-factor"))
                .drag_released()
            {
                self.build_image = true;
            }
        });

        if self.build_image {
            let time = Instant::now();
            let image = ImageBuffer::from_fn(
                app.window_rect().w() as u32,
                app.window_rect().h() as u32,
                |x, y| self.pixel_for(x, y),
            );
            self.image = DynamicImage::ImageRgba8(image);
            self.build_image = false;
            debug!("Ex_0_8 build_image time={:?}", time.elapsed());
        }
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        let texture = Texture::from_image(app, &self.image);
        draw.texture(&texture);

        draw.to_frame(app, &frame).unwrap();
    }
}
