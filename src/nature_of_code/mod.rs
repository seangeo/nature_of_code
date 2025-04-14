use nannou::prelude::*;
use nannou_egui::{self, Egui, egui};
use std::cell::Cell;

pub mod ex_0_1;
pub mod ex_0_3;
pub mod ex_0_4;
pub mod exercise;

pub fn run() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    egui: Egui,
    clear: Cell<bool>,
    exercise: Option<Box<dyn exercise::Exercise>>,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .title("Nature of Code")
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model {
        egui,
        clear: Cell::new(true),
        exercise: None, //Some(ex_0_1::init(app)),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::SidePanel::left("Exercises")
        .resizable(false)
        .show(&ctx, |ui| {
            ui.heading("Exercises");
            ui.collapsing("Chapter 0", |ui| {
                if ui.link("Exercise 0.1").clicked() {
                    model.exercise = Some(ex_0_1::init(app));
                    model.clear.set(true);
                }
                if ui.link("Exercise 0.3").clicked() {
                    model.exercise = Some(ex_0_3::init(app));
                    model.clear.set(true);
                }
                if ui.link("Exercise 0.4").clicked() {
                    model.exercise = Some(ex_0_4::init(app));
                    model.clear.set(true);
                }
            });
        });

    match &mut model.exercise {
        Some(exercise) => {
            exercise.update(app, update, &ctx);
        }
        None => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    if frame.nth() == 0 || model.clear.get() {
        frame.clear(WHITE);
        model.clear.set(false);
    }

    match &model.exercise {
        Some(exercise) => {
            exercise.draw(app, &frame);
        }
        None => {}
    }
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}
