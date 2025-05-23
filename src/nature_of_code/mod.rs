use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use std::{cell::Cell, time::Instant};

mod chapter;
mod chapter_0;
mod chapter_1;
mod exercise;
mod flow_fields;
mod noise_config;

use chapter::Chapter;
use exercise::ExerciseInfo;

pub fn run() {
    nannou::app(model).update(update).run();
}

type ExerciseSelection = Option<ExerciseInfo>;

struct Model {
    egui: Egui,
    clear: Cell<bool>,
    exercise: Option<Box<dyn exercise::Exercise>>,
    chapters: Vec<Chapter>,
    selected_exercise: Option<ExerciseInfo>,
    show_sidebar: bool,
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
        chapters: vec![
            chapter_0::chapter(),
            chapter_1::chapter(),
            flow_fields::chapter(),
        ],
        selected_exercise: None,
        show_sidebar: true,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let mut selected_exercise: ExerciseSelection = None;
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::TopBottomPanel::top("top_panel").show(&ctx, |ui| {
        ui.horizontal(|ui| {
            if ui
                .button(if model.show_sidebar {
                    "Hide Sidebar"
                } else {
                    "Show Sidebar"
                })
                .clicked()
            {
                model.show_sidebar = !model.show_sidebar;
                model.clear.set(true);
            }

            ui.add_space(8.0);
            if let Some(exercise) = &model.selected_exercise {
                ui.label(egui::RichText::new(exercise.name).strong());
            }
        });
    });

    if model.show_sidebar {
        egui::SidePanel::left("Exercises")
            .resizable(true)
            .show(&ctx, |ui| {
                ui.set_min_width(160.0);
                ui.heading("Exercises");

                for chapter in &model.chapters {
                    ui.collapsing(chapter.name, |ui| {
                        for exercise_info in chapter.exercises.iter() {
                            let is_selected = model
                                .selected_exercise
                                .map_or(false, |selected| selected.name == exercise_info.name);

                            if is_selected {
                                ui.visuals_mut().selection.bg_fill =
                                    egui::Color32::from_rgb(66, 150, 250);
                            }

                            let response = ui.selectable_label(is_selected, exercise_info.name);
                            if response.clicked() {
                                selected_exercise = Some(*exercise_info);
                            }
                        }
                    });
                }
            });
    }

    match selected_exercise {
        None => {}
        Some(exercise) => {
            let time = Instant::now();
            model.exercise = Some((exercise.init_fn)(app));
            model.selected_exercise = Some(exercise);
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
