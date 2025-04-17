use crate::nature_of_code::exercise::Exercise;
use nannou::image::DynamicImage;
use nannou::prelude::*;
use nannou::wgpu::Texture;
use nannou::{image::ImageBuffer, noise::NoiseFn};
use nannou_egui::{self, FrameCtx};

pub fn init(_app: &App) -> Box<dyn Exercise> {
    let image = ImageBuffer::new(0, 0);

    Box::new(Model {
        image: DynamicImage::ImageRgba8(image),
        build_image: true,
        noise: nannou::noise::Perlin::new(),
    })
}

struct Model {
    image: DynamicImage,
    build_image: bool,
    noise: nannou::noise::Perlin,
}

impl Model {
    fn pixel_for(&self, x: u32, y: u32) -> nannou::image::Rgba<u8> {
        let n = self
            .noise
            .get([x as f64 / 50. as f64, y as f64 / 50. as f64, 0.]);
        let c = map_range(n, -1., 1., 0, 255) as u8;
        nannou::image::Rgba([c, c, c, 100])
    }
}

impl Exercise for Model {
    fn update(&mut self, app: &App, _update: Update, _ui_ctx: &FrameCtx) {
        if self.build_image {
            let image = ImageBuffer::from_fn(
                app.window_rect().w() as u32,
                app.window_rect().h() as u32,
                |x, y| self.pixel_for(x, y),
            );
            self.image = DynamicImage::ImageRgba8(image);
            self.build_image = false;
        }
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();

        let texture = Texture::from_image(app, &self.image);
        draw.texture(&texture);

        draw.to_frame(app, &frame).unwrap();
    }
}
