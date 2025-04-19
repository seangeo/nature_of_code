use crate::nature_of_code::exercise::Exercise;
use nannou::{
    noise::{BasicMulti, NoiseFn},
    prelude::*,
};
use nannou_egui::{self, FrameCtx};

pub fn init(_app: &App) -> Box<dyn Exercise> {
    Box::new(Model { rotation: 0.0 })
}

struct Model {
    rotation: f32,
}

impl Model {
    fn build_grid(&self) -> Vec<geom::Quad<geom::Vec2>> {
        let grid_w = 16;
        let grid_h = 32;
        let cell_size = 20.;

        ((0 - grid_w / 2)..(grid_w / 2))
            .flat_map(|x| {
                ((0 - grid_h / 2)..(grid_h / 2)).map(move |y| {
                    let bottom_left = pt2(x as f32 * cell_size, y as f32 * cell_size);

                    geom::Quad([
                        bottom_left,
                        bottom_left + pt2(cell_size, 0.),
                        bottom_left + pt2(cell_size, cell_size),
                        bottom_left + pt2(0., cell_size),
                    ])
                })
            })
            .collect()
    }

    fn map_vertex(&self, v: Vec2, noise: &BasicMulti, t: f64) -> (Vec3, Srgb) {
        let n = noise.get([v.x as f64 / 50., v.y as f64 / 50., t / 4.]);
        let z = map_range(n, -1., 1., -30., 30.);

        (v.extend(z), self.map_colour(n))
    }

    fn map_colour(&self, n: f64) -> Srgb {
        let hue = map_range(n, -1., 1., 0., 1.);
        let hsl = hsl(1. - hue, 1., 0.5);
        let (r, g, b) = Rgb::from(hsl).into_components();
        srgb(r, g, b)
    }
}

impl Exercise for Model {
    fn update(&mut self, _app: &App, _update: Update, _ui_ctx: &FrameCtx) {
        self.rotation += 0.0025;

        if self.rotation > TAU {
            self.rotation -= TAU;
        }
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();
        draw.background().color(WHITE);

        let rotation_matrix = Mat4::from_rotation_y(self.rotation);
        let window = app.window_rect();
        let aspect_ratio = window.w() / window.h();
        let fov = PI / 3.0;
        let near = 10.1;
        let far = 500.0;

        let perspective_matrix = Mat4::perspective_rh_gl(fov, aspect_ratio, near, far);

        let eye = pt3(0., -20., 3.);
        let target = pt3(0., 0., 0.);
        let up = vec3(0., 1., 0.);
        let view_matrix = Mat4::look_at_rh(eye, target, up);

        let transform = perspective_matrix * rotation_matrix * view_matrix;

        let noise = BasicMulti::new();
        let grid = self.build_grid();

        let tris = grid
            .iter()
            .flat_map(|cell| cell.triangles_iter())
            .map(|tr| tr.map_vertices(|v| self.map_vertex(v, &noise, app.time as f64)));

        draw.transform(transform).mesh().tris_colored(tris);

        draw.to_frame(app, &frame).unwrap();
    }
}
