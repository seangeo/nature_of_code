use nannou::prelude::*;
use nannou_egui::FrameCtx;

pub trait Exercise {
    fn update(&mut self, app: &App, update: Update, ui_ctx: &FrameCtx);
    fn draw(&self, app: &App, frame: &Frame);
}
