use crate::math::Vec3;

const MOVE_DISTANCE: f32 = 0.5;

pub(crate) struct World {
    x_width: u32,
    z_depth: u32,
    camera: Camera,
}

impl World {
    pub(crate) fn new(x_width: u32, z_depth: u32) -> Self {
        let camera = Camera {
            position: Vec3(0.0, 0.0, 0.0),
        };

        Self {
            x_width,
            z_depth,
            camera,
        }
    }

    pub(crate) fn block_positions(&self) -> impl Iterator<Item = Vec3> {
        let x_start = 0;
        let x_end = self.x_width;
        let z_start = 0;
        let z_end = self.z_depth;

        (x_start..x_end)
            .flat_map(move |x| (z_start..z_end).map(move |z| Vec3(x as f32, -2.0, z as f32)))
    }

    pub(crate) fn move_forward(&mut self) {
        self.camera.position = self.camera.position + Vec3(0.0, 0.0, MOVE_DISTANCE);
    }

    pub(crate) fn move_backward(&mut self) {
        self.camera.position = self.camera.position + Vec3(0.0, 0.0, -MOVE_DISTANCE);
    }

    pub(crate) fn move_left(&mut self) {
        self.camera.position = self.camera.position + Vec3(-MOVE_DISTANCE, 0.0, 0.0);
    }

    pub(crate) fn move_right(&mut self) {
        self.camera.position = self.camera.position + Vec3(MOVE_DISTANCE, 0.0, 0.0);
    }

    pub(crate) fn move_up(&mut self) {
        self.camera.position = self.camera.position + Vec3(0.0, MOVE_DISTANCE, 0.0);
    }

    pub(crate) fn move_down(&mut self) {
        self.camera.position = self.camera.position + Vec3(0.0, -MOVE_DISTANCE, 0.0);
    }

    pub(crate) fn camera(&self) -> &Camera {
        &self.camera
    }
}

pub(crate) struct Camera {
    position: Vec3,
}

impl Camera {
    pub(crate) fn position(&self) -> &Vec3 {
        &self.position
    }
}
