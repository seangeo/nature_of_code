use nannou::prelude::*;

pub fn run() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    x: f32,
    y: f32,
}

fn model(_app: &App) -> Model {
    Model { x: 0., y: 0. }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let step: f32 = nannou::rand::random();

    if step >= 0.5 {
        let mouse_x = app.mouse.x;
        let mouse_y = app.mouse.y;
        if mouse_x != 0. {
            let mouse_vector = vec2(mouse_x, mouse_y);
            let model_vector = vec2(model.x, model.y);
            let direction = (mouse_vector - model_vector).normalize();

            model.x += direction.x;
            model.y += direction.y;
        }
    } else if step >= 0.4375 {
        model.x -= 1.;
        model.y -= 1.;
    } else if step >= 0.375 {
        model.x -= 1.;
    } else if step >= 0.3125 {
        model.x -= 1.;
        model.y += 1.;
    } else if step >= 0.25 {
        model.y += 1.;
    } else if step >= 0.1875 {
        model.x += 1.;
        model.y += 1.;
    } else if step >= 0.125 {
        model.x += 1.;
    } else if step >= 0.0625 {
        model.x += 1.;
        model.y -= 1.;
    } else {
        model.y -= 1.;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }

    draw.rect().x(model.x).y(model.y).w(1.).h(1.).color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
