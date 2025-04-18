use std::time::Instant;

use crate::nature_of_code::exercise::Exercise;
use nannou::image::DynamicImage;
use nannou::noise::{BasicMulti, MultiFractal};
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
        x_fact: 50.,
        y_fact: 50.,
        color_palette: ColorPalette::FullColor,
        octaves: BasicMulti::DEFAULT_OCTAVES,
        frequency: BasicMulti::DEFAULT_FREQUENCY,
        lacunarity: BasicMulti::DEFAULT_LACUNARITY,
        persistence: BasicMulti::DEFAULT_PERSISTENCE,
    })
}

#[derive(PartialEq)]
enum ColorPalette {
    Grayscale,
    FullColor,
}

struct Model {
    image: DynamicImage,
    build_image: bool,
    x_fact: f64,
    y_fact: f64,
    color_palette: ColorPalette,
    octaves: usize,
    frequency: f64,
    lacunarity: f64,
    persistence: f64,
}

impl Model {
    fn noise(&self) -> BasicMulti {
        BasicMulti::new()
            .set_octaves(self.octaves)
            .set_frequency(self.frequency)
            .set_lacunarity(self.lacunarity)
            .set_persistence(self.persistence)
    }

    fn pixel_for(&self, x: u32, y: u32, noise: &BasicMulti) -> nannou::image::Rgba<u8> {
        let n = noise.get([x as f64 / self.x_fact as f64, y as f64 / self.y_fact as f64]);

        match self.color_palette {
            ColorPalette::Grayscale => {
                let c = map_range(n, -1., 1., 0, 255) as u8;
                nannou::image::Rgba([c, c, c, 100])
            }
            ColorPalette::FullColor => {
                let hue = map_range(n, -1., 1., 0., 1.);
                let hsl = hsl(hue, 1., 0.5);
                let (r, g, b) = Rgb::from(hsl).into_components();
                nannou::image::Rgba([(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8, 100])
            }
        }
    }

    fn build_image(&mut self, width: u32, height: u32) {
        if self.build_image {
            let noise = self.noise();
            let time = Instant::now();

            let coords: Vec<(u32, u32)> = (0..height)
                .flat_map(|y| (0..width).map(move |x| (x, y)))
                .collect();

            let pixels: Vec<nannou::image::Rgba<u8>> = coords
                .par_iter()
                .map(|&(x, y)| self.pixel_for(x, y, &noise))
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
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(0., 0.),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        if ui
                            .add(egui::Slider::new(&mut self.x_fact, 0.1..=200.0).text("x-factor"))
                            .drag_released()
                        {
                            self.build_image = true;
                        }
                        if ui
                            .add(egui::Slider::new(&mut self.y_fact, 0.1..=200.0).text("y-factor"))
                            .drag_released()
                        {
                            self.build_image = true;
                        }
                    },
                );

                ui.separator();

                ui.allocate_ui_with_layout(
                    egui::Vec2::new(100., 0.),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        if ui
                            .radio_value(
                                &mut self.color_palette,
                                ColorPalette::Grayscale,
                                "Grayscale",
                            )
                            .changed()
                        {
                            self.build_image = true
                        }

                        if ui
                            .radio_value(
                                &mut self.color_palette,
                                ColorPalette::FullColor,
                                "Full Color",
                            )
                            .changed()
                        {
                            self.build_image = true
                        }
                    },
                );

                ui.separator();

                ui.allocate_ui_with_layout(
                    egui::Vec2::new(0., 0.),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        if ui
                            .add(egui::Slider::new(&mut self.octaves, 1..=10).text("Octaves"))
                            .drag_released()
                        {
                            self.build_image = true;
                        }

                        if ui
                            .add(
                                egui::Slider::new(&mut self.frequency, 0.1..=4.0).text("Frequency"),
                            )
                            .drag_released()
                        {
                            self.build_image = true;
                        }
                    },
                );

                ui.separator();

                ui.allocate_ui_with_layout(
                    egui::Vec2::new(0., 0.),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        if ui
                            .add(
                                egui::Slider::new(&mut self.lacunarity, 0.1..=4.0)
                                    .text("Lacunarity"),
                            )
                            .drag_released()
                        {
                            self.build_image = true;
                        }

                        if ui
                            .add(
                                egui::Slider::new(&mut self.persistence, 0.1..=4.0)
                                    .text("Persistence"),
                            )
                            .drag_released()
                        {
                            self.build_image = true;
                        }
                    },
                );
            });
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
