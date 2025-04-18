use std::time::Instant;

use crate::nature_of_code::exercise::Exercise;
use nannou::image::DynamicImage;
use nannou::prelude::*;
use nannou::wgpu::Texture;
use nannou::{image::ImageBuffer, noise::NoiseFn};
use nannou_egui::{self, egui, FrameCtx};
use rayon::prelude::*;

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

    fn build_image(&mut self, width: u32, height: u32) {
        if self.build_image {
            let time = Instant::now();

            let coords: Vec<(u32, u32)> = (0..height)
                .flat_map(|y| (0..width).map(move |x| (x, y)))
                .collect();

            let pixels: Vec<nannou::image::Rgba<u8>> = coords
                .par_iter()
                .map(|&(x, y)| self.pixel_for(x, y))
                .collect();

            let mut image = ImageBuffer::new(width, height);
            for (i, pixel) in pixels.iter().enumerate() {
                let x = (i % width as usize) as u32;
                let y = (i / width as usize) as u32;
                image.put_pixel(x, y, *pixel);
            }

            self.image = DynamicImage::ImageRgba8(image);
            self.build_image = false;
            debug!("Ex_0_8 build_image time={:?}", time.elapsed());
        }
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

        self.build_image(app.window_rect().w() as u32, app.window_rect().h() as u32);
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        let texture = Texture::from_image(app, &self.image);
        draw.texture(&texture);

        draw.to_frame(app, &frame).unwrap();
    }
}
