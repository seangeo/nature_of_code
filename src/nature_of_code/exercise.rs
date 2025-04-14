use nannou::prelude::*;

pub trait Exercise {
    fn update(&mut self, app: &App, update: Update);
    fn draw(&self, app: &App, frame: &Frame);
}
