use crate::math::{self, RandomNumberGenerator, Vec2, Vec3};

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

        let heightmap = Heightmap::new(16.0, 16, 16);
        let min_height = 1;
        let height_range = y_height - min_height;

        let mut heights = Vec::with_capacity(xz_area);
        for x in 0..x_width {
            for z in 0..z_depth {
                let coordinate = Coordinates(x, 0, z);
                let xz_position = coordinate.center().xz();

                let height = heightmap.height_at(&xz_position);
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
///
/// For now, the heightmap works on local coordinates from 0..CHUNK_SIDE_LENGTH on the X and Z
/// axis. The returned height must be within 0..CHUNK_SIDE_LENGTH.
#[derive(Debug)]
struct Heightmap {
    cell_size: f32,
    num_x_cells: u32,
    num_z_cells: u32,
    gradients: Vec<Vec2>,
}

impl Heightmap {
    /// Constructs a Perlin noise grid with random gradients.
    fn new(cell_size: f32, num_x_cells: u32, num_z_cells: u32) -> Self {
        let mut rng = RandomNumberGenerator::with_seed(32131);

        let num_values = ((num_z_cells + 1) * (num_x_cells + 1)) as usize;
        let mut gradients = Vec::with_capacity(num_values);

        for _ in 0..num_values {
            let angle = rng.gen_f32() * std::f32::consts::PI * 2.0;
            gradients.push(Vec2::from_angle(angle));
        }

        Self {
            cell_size,
            num_x_cells,
            num_z_cells,
            gradients,
        }
    }

    /// Constructs a Perlin noise grid with the given gradients.
    ///
    /// The number of gradients must match the number of corners in the Perlin grid. The first
    /// gradient is used for x_min / z_min in the grid. The next gradient moves along the x-axis
    /// first until it reaches x_max, then it moves onto the next row in the z axis. This function
    /// asserts that enough gradients are given to fit all of the grid corners.
    #[cfg(test)]
    fn with_gradients(
        gradients: Vec<Vec2>,
        cell_size: f32,
        num_x_cells: u32,
        num_z_cells: u32,
    ) -> Self {
        let expected_num_gradients = (num_x_cells + 1) * (num_z_cells + 1);

        assert_eq!(
            gradients.len(),
            expected_num_gradients as usize,
            "wrong number of gradients for heightmap"
        );

        Self {
            cell_size,
            num_x_cells,
            num_z_cells,
            gradients,
        }
    }

    fn height_at(&self, xz_position: &Vec2) -> f32 {
        if self.is_out_of_range(xz_position) {
            return 0.0;
        }

        let normalized_position = self.normalize_position(xz_position);

        let x0z0_height = self.height_at_x0z0(&normalized_position);
        let x0z1_height = self.height_at_x0z1(&normalized_position);
        let x1z0_height = self.height_at_x1z0(&normalized_position);
        let x1z1_height = self.height_at_x1z1(&normalized_position);

        let x_frac = normalized_position.0.fract();
        let z_frac = normalized_position.1.fract();

        let x0_height = math::interpolate(x0z0_height, x0z1_height, z_frac);
        let x1_height = math::interpolate(x1z0_height, x1z1_height, z_frac);

        let height = math::interpolate(x0_height, x1_height, x_frac);
        (height + 1.0) * 0.5
    }

    fn is_out_of_range(&self, xz_position: &Vec2) -> bool {
        let Vec2(x, z) = *xz_position;

        x < self.min_x() || x >= self.max_x() || z < self.min_z() || z >= self.max_z()
    }

    fn normalize_position(&self, xz_position: &Vec2) -> Vec2 {
        *xz_position / self.cell_size
    }

    fn min_x(&self) -> f32 {
        0.0
    }

    fn max_x(&self) -> f32 {
        self.cell_size * self.num_x_cells as f32
    }

    fn min_z(&self) -> f32 {
        0.0
    }

    fn max_z(&self) -> f32 {
        self.cell_size * self.num_z_cells as f32
    }

    fn height_at_x0z0(&self, normalized_position: &Vec2) -> f32 {
        let Vec2(x, z) = *normalized_position;

        let xi = x as usize;
        let zi = z as usize;

        let corner_position = Vec2(xi as f32, zi as f32);
        let gradient = self.gradient_at_index(xi, zi);

        let offset = *normalized_position - corner_position;
        gradient.dot(&offset)
    }

    fn height_at_x1z0(&self, normalized_position: &Vec2) -> f32 {
        let Vec2(x, z) = *normalized_position;

        let xi = x as usize + 1;
        let zi = z as usize;

        let corner_position = Vec2(xi as f32, zi as f32);
        let gradient = self.gradient_at_index(xi, zi);

        let offset = *normalized_position - corner_position;
        gradient.dot(&offset)
    }

    fn height_at_x0z1(&self, normalized_position: &Vec2) -> f32 {
        let Vec2(x, z) = *normalized_position;

        let xi = x as usize;
        let zi = z as usize + 1;

        let corner_position = Vec2(xi as f32, zi as f32);
        let gradient = self.gradient_at_index(xi, zi);

        let offset = *normalized_position - corner_position;
        gradient.dot(&offset)
    }

    fn height_at_x1z1(&self, normalized_position: &Vec2) -> f32 {
        let Vec2(x, z) = *normalized_position;

        let xi = x as usize + 1;
        let zi = z as usize + 1;

        let corner_position = Vec2(xi as f32, zi as f32);
        let gradient = self.gradient_at_index(xi, zi);

        let offset = *normalized_position - corner_position;
        gradient.dot(&offset)
    }

    fn gradient_at_index(&self, xi: usize, zi: usize) -> Vec2 {
        let i = zi * self.num_x_cells as usize + xi;
        self.gradients[i]
    }
}

struct Coordinates(u32, u32, u32);

impl Coordinates {
    fn center(&self) -> Vec3 {
        let Self(x, y, z) = *self;
        Vec3(x as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::*;

    use super::*;

    #[test]
    fn perlin_noise_single_grid_cell() {
        // Create a 2D Perlin noise grid with one cell and four gradients (one for each corner). In
        // this example, all gradients are unit vectors that point along either the X or Z axis.
        let gradients = vec![
            Vec2::from_angle(0.0),
            Vec2::from_angle(-PI),
            Vec2::from_angle(FRAC_PI_2),
            Vec2::from_angle(-FRAC_PI_2),
        ];

        let heightmap = Heightmap::with_gradients(gradients, 1.0, 1, 1);

        let examples = [
            // Always zero at the grid corner
            (Vec2(0.0, 0.0), 0.0),
            // Halfway between two corners on X axis, both with gradients facing inward (max strength)
            (Vec2(0.5, 0.0), 0.5),
            // Strength drops as approaching either corner. Should drop equally on both sides.
            (Vec2(0.25, 0.0), 0.375),
            (Vec2(0.75, 0.0), 0.375),
            // Along the Z axis, gradient faces away so should be negative
            (Vec2(0.0, 0.5), -0.25),
            (Vec2(0.0, 0.25), -0.1875),
            (Vec2(0.0, 0.75), -0.1875),
            // In the middle. The two gradients along Z=0 contribute 0.5, and the two gradients at
            // Z=1 are facing opposite directions and cancel each other out (so 0.0), which results
            // in an interpolated value of 0.25.
            (Vec2(0.5, 0.5), 0.25),
        ];

        for (position, expected) in examples.into_iter() {
            let actual = heightmap.height_at(&position);

            assert!((actual - expected).abs() < 0.00001);
        }
    }

    #[test]
    fn perlin_noise_varying_cell_size() {
        let small_cell_heightmap = Heightmap::new(1.0, 2, 2);
        let big_cell_heightmap = Heightmap::new(16.0, 2, 2);

        // Positions for the small cell and big cell heightmaps. The left and right values should
        // return the same height value since they're in the same spot relative to the size of the
        // grid.
        let examples = [
            (Vec2(0.0, 0.0), Vec2(0.0, 0.0)),
            (Vec2(1.5, 0.5), Vec2(24.0, 8.0)),
            (Vec2(1.0, 1.0), Vec2(16.0, 16.0)),
        ];

        for (small_position, big_position) in examples.into_iter() {
            let small_cell_height = small_cell_heightmap.height_at(&small_position);
            let big_cell_height = big_cell_heightmap.height_at(&big_position);

            assert_eq!(small_cell_height, big_cell_height);
        }
    }

    #[test]
    fn perlin_noise_outside_of_cell_range() {
        let heightmap = Heightmap::new(16.0, 2, 2);

        // Positions outside of the grid range should return 0.0 height.
        let examples = [
            Vec2(-0.0001, 0.0),
            Vec2(32.0, 32.0),
            Vec2(32.0, 0.0),
            Vec2(16.0, 32.0),
            Vec2(-0.0001, 10.0),
        ];

        for position in examples.into_iter() {
            assert_eq!(heightmap.height_at(&position), 0.0);
        }
    }
}
