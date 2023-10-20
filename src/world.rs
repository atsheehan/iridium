use crate::math::Vec3;

const MOUSE_SENSITIVITY: f32 = 0.001;
const MOVE_SPEED: f32 = 0.5;

pub(crate) struct World {
    x_width: u32,
    y_height: u32,
    z_depth: u32,
    camera: Camera,
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

        Self {
            x_width,
            y_height,
            z_depth,
            camera,
        }
    }

    pub(crate) fn update(&mut self) {
        let actual_velocity = self.camera.velocity.rotate_y(self.camera.heading);
        self.camera.position = self.camera.position + actual_velocity;
    }

    pub(crate) fn block_positions(&self) -> impl Iterator<Item = Vec3> {
        let x_start = 0;
        let x_end = self.x_width;
        let y_start = 0;
        let y_end = self.y_height;
        let z_start = 0;
        let z_end = self.z_depth;

        (x_start..x_end).flat_map(move |x| {
            (y_start..y_end).flat_map(move |y| {
                (z_start..z_end).map(move |z| Vec3(x as f32, y as f32, z as f32))
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
