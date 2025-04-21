use crate::nature_of_code::exercise::Exercise;
use nannou::prelude::*;
use nannou_egui::{self, FrameCtx};

pub fn init(app: &App) -> Box<dyn Exercise> {
    Box::new(model(app))
}

struct Ball {
    position: Vec3,
    velocity: Vec3,
    radius: f32,
}

struct Model {
    rotation: f32,
    bounds: Cuboid,
    ball: Ball,
}

impl Model {
    fn bounds_vertices(&self) -> impl Iterator<Item = Vec3> {
        self.bounds.corners_iter().map(|c| vec3(c[0], c[1], c[2]))
    }

    fn bound_edge_indices(&self) -> Vec<(usize, usize)> {
        vec![
            (4, 5),
            (5, 7),
            (7, 6),
            (6, 4),
            (0, 1),
            (1, 3),
            (3, 2),
            (2, 0),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ]
    }
}

fn model(_app: &App) -> Model {
    Model {
        bounds: Cuboid::from_xyz_whd(Vec3::ZERO, vec3(200., 150., 200.)),
        rotation: 0.0,
        ball: Ball {
            position: Vec3::ZERO,
            velocity: Vec3::splat(2.),
            radius: 40.,
        },
    }
}

impl Exercise for Model {
    fn update(&mut self, _app: &App, _update: Update, _ui_ctx: &FrameCtx) {
        self.rotation += 0.01;

        let ball_rect =
            Cuboid::from_xyz_whd(self.ball.position, Vec3::splat(self.ball.radius * 2.));

        if ball_rect.top() >= self.bounds.top() || ball_rect.bottom() <= self.bounds.bottom() {
            self.ball.velocity *= vec3(1., -1., 1.);
        }

        if ball_rect.left() <= self.bounds.left() || ball_rect.right() >= self.bounds.right() {
            self.ball.velocity *= vec3(-1., 1., 1.);
        }

        if ball_rect.front() <= self.bounds.front() || ball_rect.back() >= self.bounds.back() {
            self.ball.velocity *= vec3(1., 1., -1.);
        }

        self.ball.position += self.ball.velocity;
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();
        frame.clear(WHITE);

        // Get window dimensions
        let win = app.window_rect();
        let aspect_ratio = win.w() / win.h();
        let fov = PI / 4.;
        let near = 0.1;
        let far = 1000.;
        let position = vec3(0., 0., 500.);
        let up = Vec3::Y;
        let target = Vec3::ZERO;

        let view = Mat4::look_to_rh(position, target - position, up);

        // Create a projection matrix
        let projection = Mat4::perspective_rh(fov, aspect_ratio, near, far);

        // Create a model matrix for rotation
        let model =
            Mat4::from_rotation_y(self.rotation) * Mat4::from_rotation_x(self.rotation * 0.5);

        // Combined transformation matrix
        let mvp = projection * view * model;

        // Project 3D vertices to 2D
        let projected: Vec<Vec2> = self
            .bounds_vertices()
            .map(|v| {
                // Apply model-view-projection transformation
                let clip_space = mvp * Vec4::new(v.x, v.y, v.z, 1.0);

                // Perspective divide
                let ndc = Vec3::new(
                    clip_space.x / clip_space.w,
                    clip_space.y / clip_space.w,
                    clip_space.z / clip_space.w,
                );

                // Convert to screen coordinates
                let screen_x = (ndc.x + 1.0) * 0.5 * win.w() - win.w() / 2.0;
                let screen_y = (1.0 - ndc.y) * 0.5 * win.h() - win.h() / 2.0;

                Vec2::new(screen_x, screen_y)
            })
            .collect();

        // Draw edges
        for (i, j) in self.bound_edge_indices() {
            draw.line()
                .start(projected[i])
                .end(projected[j])
                .color(BLACK)
                .weight(2.0);
        }

        {
            // ball
            let transformed_position = model.transform_point3(self.ball.position);
            let clip_space = mvp * self.ball.position.extend(1.0);

            // Perspective divide
            let ndc = Vec3::new(
                clip_space.x / clip_space.w,
                clip_space.y / clip_space.w,
                clip_space.z / clip_space.w,
            );

            // Convert to screen coordinates
            let screen_x = (ndc.x + 1.0) * 0.5 * win.w() - win.w() / 2.0;
            let screen_y = (1.0 - ndc.y) * 0.5 * win.h() - win.h() / 2.0;

            let distance_from_camera = 500. - transformed_position.z;

            let scale_factor = if distance_from_camera > 0. {
                150. / distance_from_camera
            } else {
                0.5
            };

            draw.ellipse()
                .x_y(screen_x, screen_y)
                .radius(self.ball.radius * scale_factor)
                .color(GRAY)
                .stroke_weight(1.)
                .stroke(BLACK);
        }

        draw.to_frame(app, &frame).unwrap();
    }
}
