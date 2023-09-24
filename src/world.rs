use crate::math::{Vec2, Vec3};

const MOUSE_SENSITIVITY: f32 = 0.01;
const MOVE_SPEED: f32 = 0.5;

pub(crate) struct World {
    x_width: u32,
    z_depth: u32,
    camera: Camera,
    heights: Vec<u32>,
}

impl World {
    pub(crate) fn new(x_width: u32, y_height: u32, z_depth: u32) -> Self {
        let starting_position = Vec3((x_width / 2) as f32, (y_height + 1) as f32, 0.0);

        let camera = Camera {
            position: starting_position,
            velocity: Vec3(0.0, 0.0, 0.0),
            heading: 0.0,
            pitch: 0.0,
        };

        let xz_area = (x_width * z_depth) as usize;

        let heightmap = Heightmap::new(Vec2((x_width / 2) as f32, (z_depth / 2) as f32), 25.0);
        let min_height = 1;
        let height_range = y_height - min_height;

        let mut heights = Vec::with_capacity(xz_area);
        for x in 0..x_width {
            for z in 0..z_depth {
                let coordinate = Coordinates(x, 0, z);
                let xz_position = coordinate.center().xz();

                let height = heightmap.height_at(xz_position.0, xz_position.1);
                let scaled_height = (height * height_range as f32) as u32 + min_height;

                heights.push(scaled_height);
            }
        }

        Self {
            x_width,
            z_depth,
            camera,
            heights,
        }
    }

    pub(crate) fn update(&mut self) {
        let actual_velocity = self.camera.velocity.rotate_y(self.camera.heading);
        self.camera.position = self.camera.position + actual_velocity;
    }

    pub(crate) fn block_positions(&self) -> impl Iterator<Item = Vec3> + '_ {
        (0..self.z_depth).flat_map(move |z| {
            (0..self.x_width).flat_map(move |x| {
                let index = ((self.x_width * z) + x) as usize;
                let y_max = self.heights[index];

                (0..y_max).map(move |y| Vec3(x as f32, y as f32, z as f32))
            })
        })
    }

    pub(crate) fn start_moving_forward(&mut self) {
        self.camera.velocity = self.camera.velocity.set_z(MOVE_SPEED);
    }

    pub(crate) fn stop_moving_forward(&mut self) {
        self.camera.velocity = self.camera.velocity.set_z(0.0);
    }

    pub(crate) fn start_moving_backward(&mut self) {
        self.camera.velocity = self.camera.velocity.set_z(-MOVE_SPEED);
    }

    pub(crate) fn stop_moving_backward(&mut self) {
        self.camera.velocity = self.camera.velocity.set_z(0.0);
    }

    pub(crate) fn start_moving_left(&mut self) {
        self.camera.velocity = self.camera.velocity.set_x(-MOVE_SPEED);
    }

    pub(crate) fn stop_moving_left(&mut self) {
        self.camera.velocity = self.camera.velocity.set_x(0.0);
    }

    pub(crate) fn start_moving_right(&mut self) {
        self.camera.velocity = self.camera.velocity.set_x(MOVE_SPEED);
    }

    pub(crate) fn stop_moving_right(&mut self) {
        self.camera.velocity = self.camera.velocity.set_x(0.0);
    }

    pub(crate) fn start_moving_up(&mut self) {
        self.camera.velocity = self.camera.velocity.set_y(MOVE_SPEED);
    }

    pub(crate) fn stop_moving_up(&mut self) {
        self.camera.velocity = self.camera.velocity.set_y(0.0);
    }

    pub(crate) fn start_moving_down(&mut self) {
        self.camera.velocity = self.camera.velocity.set_y(-MOVE_SPEED);
    }

    pub(crate) fn stop_moving_down(&mut self) {
        self.camera.velocity = self.camera.velocity.set_y(0.0);
    }

    pub(crate) fn update_camera_direction(&mut self, dx: f32, dy: f32) {
        const MIN_PITCH: f32 = -std::f32::consts::FRAC_PI_2;
        const MAX_PITCH: f32 = std::f32::consts::FRAC_PI_2;

        self.camera.heading += dx * MOUSE_SENSITIVITY;
        self.camera.pitch =
            (self.camera.pitch + dy * MOUSE_SENSITIVITY).clamp(MIN_PITCH, MAX_PITCH);
    }

    pub(crate) fn camera(&self) -> &Camera {
        &self.camera
    }
}

pub(crate) struct Camera {
    position: Vec3,
    velocity: Vec3,
    heading: f32,
    pitch: f32,
}

impl Camera {
    pub(crate) fn position(&self) -> &Vec3 {
        &self.position
    }

    pub(crate) fn heading(&self) -> f32 {
        self.heading
    }

    pub(crate) fn pitch(&self) -> f32 {
        self.pitch
    }
}

/// Describes how elevation varies across the x-z plane.
struct Heightmap {
    center: Vec2,
    spread: f32,
}

impl Heightmap {
    fn new(center: Vec2, spread: f32) -> Self {
        Self { center, spread }
    }

    fn height_at(&self, x: f32, z: f32) -> f32 {
        let Vec2(x_center, z_center) = self.center;
        let spread = self.spread * self.spread * 2.0;

        let dx = x_center - x;
        let dz = z_center - z;

        let x_term = (dx * dx) / spread;
        let z_term = (dz * dz) / spread;

        let sum = -(x_term + z_term);
        sum.exp()
    }
}

struct Coordinates(u32, u32, u32);

impl Coordinates {
    fn center(&self) -> Vec3 {
        let Self(x, y, z) = *self;
        Vec3(x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5)
    }
}
