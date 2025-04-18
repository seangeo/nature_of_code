use crate::nature_of_code::exercise::Exercise;
use nannou::{
    noise::{BasicMulti, NoiseFn},
    prelude::*,
};
use nannou_egui::{self, FrameCtx};

pub fn init(_app: &App) -> Box<dyn Exercise> {
    Box::new(Model {})
}

struct Model {}

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
        let c = map_range(n, -1., 1., 0., 1.);
        let color = srgb(c, c, c);

        (v.extend(z), color)
    }
}

impl Exercise for Model {
    fn update(&mut self, _app: &App, _update: Update, _ui_ctx: &FrameCtx) {}

    // width: 16 x 20 = 320
    // height: 32 x 20 = 640
    //
    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();
        draw.background().color(WHITE);

        let noise = BasicMulti::new();
        let grid = self.build_grid();

        let tris = grid
            .iter()
            .flat_map(|cell| cell.triangles_iter())
            .map(|tr| tr.map_vertices(|v| self.map_vertex(v, &noise, app.time as f64)));

        draw.scale(1.5)
            .mesh()
            .tris_colored(tris.clone())
            .x_radians(PI / 3.5)
            .z_radians(app.time / 5.);

        draw.to_frame(app, &frame).unwrap();
    }
}
