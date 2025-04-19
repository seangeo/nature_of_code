use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use std::{cell::Cell, time::Instant};

mod chapter_0;
mod exercise;

// Define exercises by chapter
const CHAPTER_0_EXERCISES: [ExerciseInfo; 8] = [
    ExerciseInfo {
        name: "Exercise 0.1",
        init_fn: chapter_0::ex_0_1::init,
    },
    ExerciseInfo {
        name: "Exercise 0.3",
        init_fn: chapter_0::ex_0_3::init,
    },
    ExerciseInfo {
        name: "Exercise 0.4",
        init_fn: chapter_0::ex_0_4::init,
    },
    ExerciseInfo {
        name: "Exercise 0.5",
        init_fn: chapter_0::ex_0_5::init,
    },
    ExerciseInfo {
        name: "Exercise 0.6",
        init_fn: chapter_0::ex_0_6::init,
    },
    ExerciseInfo {
        name: "Exercise 0.7",
        init_fn: chapter_0::ex_0_7::init,
    },
    ExerciseInfo {
        name: "Exercise 0.8",
        init_fn: chapter_0::ex_0_8::init,
    },
    ExerciseInfo {
        name: "Exercise 0.10",
        init_fn: chapter_0::ex_0_10::init,
    },
];

pub fn run() {
    nannou::app(model).update(update).run();
}

#[derive(Copy, Clone)]
struct ExerciseInfo {
    name: &'static str,
    init_fn: fn(&App) -> Box<dyn exercise::Exercise>,
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
                for exercise_info in CHAPTER_0_EXERCISES.iter() {
                    if ui.link(exercise_info.name).clicked() {
                        selected_exercise = Some(*exercise_info);
                    }
                }
            });
            
            // Future chapters can be added here
            // ui.collapsing("Chapter 1", |ui| {
            //     for exercise_info in CHAPTER_1_EXERCISES.iter() {
            //         if ui.link(exercise_info.name).clicked() {
            //             selected_exercise = Some(*exercise_info);
            //         }
            //     }
            // });
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
