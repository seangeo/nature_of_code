use crate::nature_of_code::exercise::Exercise;
use nannou::prelude::*;
use nannou_egui::{self, FrameCtx};

pub fn init(app: &App) -> Box<dyn Exercise> {
    Box::new(model(app))
}

struct Model {
    rotation: f32,
    vertices: Vec<Vec3>,
    edges: Vec<(usize, usize)>,
}

fn model(_app: &App) -> Model {
    // Define cube vertices (centered at origin)
    let half_size = 100.0;
    let vertices = vec![
        Vec3::new(-half_size, -half_size, -half_size), // 0: bottom-left-back
        Vec3::new(half_size, -half_size, -half_size),  // 1: bottom-right-back
        Vec3::new(half_size, half_size, -half_size),   // 2: top-right-back
        Vec3::new(-half_size, half_size, -half_size),  // 3: top-left-back
        Vec3::new(-half_size, -half_size, half_size),  // 4: bottom-left-front
        Vec3::new(half_size, -half_size, half_size),   // 5: bottom-right-front
        Vec3::new(half_size, half_size, half_size),    // 6: top-right-front
        Vec3::new(-half_size, half_size, half_size),   // 7: top-left-front
    ];

    // Define cube edges (indices of connected vertices)
    let edges = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0), // back face
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4), // front face
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7), // connecting edges
    ];

    Model {
        rotation: 0.0,
        vertices,
        edges,
    }
}

impl Exercise for Model {
    fn update(&mut self, _app: &App, _update: Update, _ui_ctx: &FrameCtx) {
        // Rotate the cube
        self.rotation += 0.01;
    }

    fn draw(&self, app: &App, frame: &Frame) {
        let draw = app.draw();
        frame.clear(WHITE);

        // Get window dimensions
        let win = app.window_rect();
        
        // Create a camera
        let camera = Camera3d {
            position: Vec3::new(0.0, 0.0, 500.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            fov: std::f32::consts::PI / 4.0,
            near: 0.1,
            far: 1000.0,
        };
        
        // Create a view matrix
        let view = Mat4::look_to_rh(camera.position, camera.target - camera.position, camera.up);
        
        // Create a projection matrix
        let aspect_ratio = win.w() / win.h();
        let projection = Mat4::perspective_rh(camera.fov, aspect_ratio, camera.near, camera.far);
        
        // Create a model matrix for rotation
        let model = Mat4::from_rotation_y(self.rotation) * Mat4::from_rotation_x(self.rotation * 0.5);
        
        // Combined transformation matrix
        let mvp = projection * view * model;
        
        // Project 3D vertices to 2D
        let projected: Vec<Vec2> = self
            .vertices
            .iter()
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
        for &(i, j) in &self.edges {
            draw.line()
                .start(projected[i])
                .end(projected[j])
                .color(BLACK)
                .weight(2.0);
        }

        draw.to_frame(app, &frame).unwrap();
    }
}

// Helper struct for camera parameters
struct Camera3d {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    fov: f32,
    near: f32,
    far: f32,
}
