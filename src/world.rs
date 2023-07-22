use crate::math::Vec3;

pub(crate) struct World {
    x_width: u32,
    z_depth: u32,
}

impl World {
    pub(crate) fn new(x_width: u32, z_depth: u32) -> Self {
        Self { x_width, z_depth }
    }

    pub(crate) fn block_positions(&self) -> impl Iterator<Item = Vec3> {
        let x_start = 0;
        let x_end = self.x_width;
        let z_start = 0;
        let z_end = self.z_depth;

        (x_start..x_end)
            .flat_map(move |x| (z_start..z_end).map(move |z| Vec3(x as f32, -2.0, z as f32)))
    }
}
