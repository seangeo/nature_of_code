use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use std::{cell::Cell, time::Instant};

mod chapter_0;
mod exercise;

use exercise::ExerciseInfo;

pub fn run() {
    nannou::app(model).update(update).run();
}

type ExerciseSelection = Option<ExerciseInfo>;

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
        exercise: None,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let mut selected_exercise: ExerciseSelection = None;
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::SidePanel::left("Exercises")
        .resizable(false)
        .show(&ctx, |ui| {
            ui.heading("Exercises");
            
            // Chapter 0
            ui.collapsing("Chapter 0", |ui| {
                for exercise_info in chapter_0::EXERCISES.iter() {
                    if ui.link(exercise_info.name).clicked() {
                        selected_exercise = Some(*exercise_info);
                    }
                }
            });
        });

    // After the UI is updated, check if an exercise was selected and initialize it
    match selected_exercise {
        None => {}
        Some(exercise) => {
            let time = Instant::now();
            model.exercise = Some((exercise.init_fn)(app));
            debug!("Exercise initialisation time={:?}", time.elapsed());
            model.clear.set(true);
        }
    }

    match &mut model.exercise {
        Some(exercise) => {
            let time = Instant::now();
            exercise.update(app, update, &ctx);
            debug!("Exercise update time={:?}", time.elapsed());
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
            let time = Instant::now();
            exercise.draw(app, &frame);
            debug!("Exercise draw time={:?}", time.elapsed());
        }
        None => {}
    }
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}
