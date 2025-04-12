use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use rand_distr::{Distribution, Normal};

pub fn run() {
    nannou::app(model).update(update).run();
}

struct Model {
    egui: Egui,
    splatter_stddev: f32,
    color_stddev: f32,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .title("Splatter")
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model {
        egui,
        splatter_stddev: 80.,
        color_stddev: 0.2,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::TopBottomPanel::bottom("Settings").show(&ctx, |ui| {
        ui.add(
            egui::Slider::new(&mut model.splatter_stddev, 0.0..=200.0).text("Splatter Std Dev."),
        );
        ui.add(egui::Slider::new(&mut model.color_stddev, 0.0..=1.0).text("Colour Std Dev."));
    });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }

    draw_dot(&draw, model.splatter_stddev, model.color_stddev);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn draw_dot(draw: &Draw, splatter_stddev: f32, color_stddev: f32) {
    let mut r = rand::rng();
    let normal_pos = Normal::new(0., splatter_stddev).unwrap();
    let vx = normal_pos.sample(&mut r);
    let vy = normal_pos.sample(&mut r);

    let normal_col = Normal::new(0.5, color_stddev).unwrap();
    let mut r2 = rand::rng();
    let r = normal_col.sample(&mut r2).clamp(0., 1.);
    let g = normal_col.sample(&mut r2).clamp(0., 1.);
    let b = normal_col.sample(&mut r2).clamp(0., 1.);
    let a = normal_col.sample(&mut r2).clamp(0., 1.);

    draw.ellipse().radius(5.0).rgba(r, g, b, a).x(vx).y(vy);
}
