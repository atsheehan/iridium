use crate::math::Vec3;

const MOUSE_SENSITIVITY: f32 = 0.001;
const MOVE_SPEED: f32 = 0.5;

pub(crate) struct World {
    x_width: u32,
    z_depth: u32,
    camera: Camera,
}

impl World {
    pub(crate) fn new(x_width: u32, z_depth: u32) -> Self {
        let camera = Camera {
            position: Vec3(0.0, 0.0, 0.0),
            velocity: Vec3(0.0, 0.0, 0.0),
            heading: 0.0,
        };

        Self {
            x_width,
            z_depth,
            camera,
        }
    }

    pub(crate) fn update(&mut self) {
        self.camera.position = self.camera.position + self.camera.velocity;
    }

    pub(crate) fn block_positions(&self) -> impl Iterator<Item = Vec3> {
        let x_start = 0;
        let x_end = self.x_width;
        let z_start = 0;
        let z_end = self.z_depth;

        (x_start..x_end)
            .flat_map(move |x| (z_start..z_end).map(move |z| Vec3(x as f32, -2.0, z as f32)))
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

    pub(crate) fn update_camera_direction(&mut self, dx: f32, _dy: f32) {
        self.camera.heading += dx * MOUSE_SENSITIVITY;
    }

    pub(crate) fn camera(&self) -> &Camera {
        &self.camera
    }
}

pub(crate) struct Camera {
    position: Vec3,
    velocity: Vec3,
    heading: f32,
}

impl Camera {
    pub(crate) fn position(&self) -> &Vec3 {
        &self.position
    }

    pub(crate) fn heading(&self) -> f32 {
        self.heading
    }
}
