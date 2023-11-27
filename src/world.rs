use crate::math::{self, RandomNumberGenerator, Vec2, Vec3};

const MOUSE_SENSITIVITY: f32 = 0.01;
const MOVE_SPEED: f32 = 0.5;

pub(crate) struct World {
    camera: Camera,
    terrain: Terrain,
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

        let heightmap = Heightmap::new(16.0, 16, 16);
        let terrain = Terrain::from_heightmap(heightmap, x_width, y_height, z_depth);

        Self {
            camera,
            terrain,
        }
    }

    pub(crate) fn update(&mut self) {
        let heading_velocity = self.camera.velocity.rotate_y(self.camera.heading);
        self.camera.position = self.check_for_collisions(self.camera.position, heading_velocity);
    }

    pub(crate) fn visible_block_positions(&self) -> impl Iterator<Item = Vec3> + '_ {
        self.terrain.visible_block_positions()
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

    fn check_for_collisions(&self, position: Vec3, velocity: Vec3) -> Vec3 {
        let position = self.check_for_collisions_in_x_axis(position, velocity.x());
        let position = self.check_for_collisions_in_y_axis(position, velocity.y());
        self.check_for_collisions_in_z_axis(position, velocity.z())
    }

    fn check_for_collisions_in_x_axis(&self, position: Vec3, move_distance: f32) -> Vec3 {
        const BUFFER_DISTANCE: f32 = 0.02;

        let range = GlobalIndexRange::along_x_axis(position, move_distance);

        let colliding_block = range.skip(1).find_map(|index| self.terrain.block_at(index));

        if let Some(block) = colliding_block {
            if move_distance > 0.0 {
                position.set_x(block.left() - BUFFER_DISTANCE)
            } else {
                position.set_x(block.right() + BUFFER_DISTANCE)
            }
        } else {
            position.set_x(position.x() + move_distance)
        }
    }

    fn check_for_collisions_in_y_axis(&self, position: Vec3, move_distance: f32) -> Vec3 {
        const BUFFER_DISTANCE: f32 = 0.02;

        let range = GlobalIndexRange::along_y_axis(position, move_distance);

        let colliding_block = range.skip(1).find_map(|index| self.terrain.block_at(index));

        if let Some(block) = colliding_block {
            if move_distance > 0.0 {
                position.set_y(block.bottom() - BUFFER_DISTANCE)
            } else {
                position.set_y(block.top() + BUFFER_DISTANCE)
            }
        } else {
            position.set_y(position.y() + move_distance)
        }
    }

    fn check_for_collisions_in_z_axis(&self, position: Vec3, move_distance: f32) -> Vec3 {
        const BUFFER_DISTANCE: f32 = 0.02;

        let range = GlobalIndexRange::along_z_axis(position, move_distance);

        let colliding_block = range.skip(1).find_map(|index| self.terrain.block_at(index));

        if let Some(block) = colliding_block {
            if move_distance > 0.0 {
                position.set_z(block.near() - BUFFER_DISTANCE)
            } else {
                position.set_z(block.far() + BUFFER_DISTANCE)
            }
        } else {
            position.set_z(position.z() + move_distance)
        }
    }

    pub(crate) fn destroy_block(&mut self) {
        self.destroy_block_at(GlobalIndex::from(self.camera.position.map_y(|y| y - 1.0)));
    }

    fn destroy_block_at(&mut self, position: GlobalIndex) {
        self.terrain.decrement_height_at(position.x(), position.z());
    }
}

struct Block {
    index: GlobalIndex,
}

impl Block {
    fn new(index: GlobalIndex) -> Block {
        Self { index }
    }

    fn left(&self) -> f32 {
        self.index.x() as f32
    }

    fn right(&self) -> f32 {
        (self.index.x() + 1) as f32
    }

    fn top(&self) -> f32 {
        (self.index.y() + 1) as f32
    }

    fn bottom(&self) -> f32 {
        self.index.y() as f32
    }

    fn near(&self) -> f32 {
        self.index.z() as f32
    }

    fn far(&self) -> f32 {
        (self.index.z() + 1) as f32
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

struct Terrain {
    x_width: u32,
    z_depth: u32,
    heights: Vec<i32>,
}

impl Terrain {
    fn from_heightmap(heightmap: Heightmap, x_width: u32, y_height: u32, z_depth: u32) -> Self {
        let xz_area = (x_width * z_depth) as usize;
        let min_height = 1;
        let height_range = y_height - min_height;

        let mut heights = Vec::with_capacity(xz_area);
        for x in 0..x_width {
            for z in 0..z_depth {
                let coordinate = Coordinates(x, 0, z);
                let xz_position = coordinate.center().xz();

                let height = heightmap.height_at(&xz_position);
                let scaled_height = (height * height_range as f32) as i32 + min_height as i32;

                heights.push(scaled_height);
            }
        }

        Self { heights, x_width, z_depth }
    }

    fn block_at(&self, index: GlobalIndex) -> Option<Block> {
        if self.height_at(index.x(), index.z()) >= index.y() {
            Some(Block::new(index))
        } else {
            None
        }
    }

    fn height_at(&self, x: i32, z: i32) -> i32 {
        if x < 0 || x >= self.x_width as i32 || z < 0 || z >= self.z_depth as i32 {
            0
        } else {
            let index = ((self.x_width as i32 * z) + x) as usize;
            self.heights[index]
        }
    }

    fn decrement_height_at(&mut self, x: i32, z: i32) {
        if x >= 0 && x < self.x_width as i32 && z >= 0 && z < self.z_depth as i32 {
            let index = ((self.x_width as i32 * z) + x) as usize;

            if self.heights[index] > 0 {
                self.heights[index] -= 1;
            }
        }
    }

    fn visible_block_positions(&self) -> impl Iterator<Item = Vec3> + '_ {
        let x_width = self.x_width as i32;
        let z_depth = self.z_depth as i32;

        (0..z_depth).flat_map(move |z| {
            (0..x_width).flat_map(move |x| {
                let y_max = self.height_at(x, z);

                let min_neighbor_height = [
                    self.height_at(x + 1, z),
                    self.height_at(x - 1, z),
                    self.height_at(x, z + 1),
                    self.height_at(x, z - 1),
                ]
                .into_iter()
                .min()
                .unwrap();

                let y_min = y_max.min(min_neighbor_height);

                (y_min..=y_max).map(move |y| Vec3(x as f32, y as f32, z as f32))
            })
        })
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

enum GlobalIndexRange {
    X {
        x_range: BidirectionalRange,
        y: i32,
        z: i32,
    },
    Y {
        y_range: BidirectionalRange,
        x: i32,
        z: i32,
    },
    Z {
        x: i32,
        y: i32,
        z_range: BidirectionalRange,
    },
}

impl GlobalIndexRange {
    fn along_x_axis(start: Vec3, distance: f32) -> Self {
        let start_index = GlobalIndex::from(start);
        let end_index = GlobalIndex::from(start.map_x(|x| x + distance));

        let y = start_index.y();
        let z = start_index.z();
        let x_range = BidirectionalRange::new(start_index.x(), end_index.x());

        Self::X { y, z, x_range }
    }

    fn along_y_axis(start: Vec3, distance: f32) -> Self {
        let start_index = GlobalIndex::from(start);
        let end_index = GlobalIndex::from(start.map_y(|y| y + distance));

        let x = start_index.x();
        let z = start_index.z();
        let y_range = BidirectionalRange::new(start_index.y(), end_index.y());

        Self::Y { x, z, y_range }
    }

    fn along_z_axis(start: Vec3, distance: f32) -> Self {
        let start_index = GlobalIndex::from(start);
        let end_index = GlobalIndex::from(start.map_z(|z| z + distance));

        let x = start_index.x();
        let y = start_index.y();
        let z_range = BidirectionalRange::new(start_index.z(), end_index.z());

        Self::Z { x, y, z_range }
    }
}

impl Iterator for GlobalIndexRange {
    type Item = GlobalIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::X { x_range, y, z } => x_range.next().map(|x| GlobalIndex(x, *y, *z)),
            Self::Y { x, y_range, z } => y_range.next().map(|y| GlobalIndex(*x, y, *z)),
            Self::Z { x, y, z_range } => z_range.next().map(|z| GlobalIndex(*x, *y, z)),
        }
    }
}

enum Direction {
    Ascending,
    Descending,
}

impl Direction {
    fn step(&self) -> i32 {
        match self {
            Self::Ascending => 1,
            Self::Descending => -1,
        }
    }
}

struct BidirectionalRange {
    next: i32,
    end: i32,
    direction: Direction,
}

impl BidirectionalRange {
    fn new(start: i32, end: i32) -> Self {
        let direction = if start > end {
            Direction::Descending
        } else {
            Direction::Ascending
        };

        Self {
            next: start,
            end,
            direction,
        }
    }
}

impl Iterator for BidirectionalRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.next;
        self.next += self.direction.step();

        match self.direction {
            Direction::Ascending => {
                if value <= self.end {
                    Some(value)
                } else {
                    None
                }
            }
            Direction::Descending => {
                if value >= self.end {
                    Some(value)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct GlobalIndex(i32, i32, i32);

impl GlobalIndex {
    fn x(&self) -> i32 {
        self.0
    }

    fn y(&self) -> i32 {
        self.1
    }

    fn z(&self) -> i32 {
        self.2
    }
}

impl From<Vec3> for GlobalIndex {
    fn from(value: Vec3) -> Self {
        let x = value.x().floor() as i32;
        let y = value.y().floor() as i32;
        let z = value.z().floor() as i32;

        Self(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::*;

    use super::*;

    #[test]
    #[ignore]
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

    #[test]
    fn converting_vec3_into_global_index() {
        let examples = [
            (Vec3(0.0, 0.0, 0.0), GlobalIndex(0, 0, 0)),
            (Vec3(1.0, 2.8, 3.5), GlobalIndex(1, 2, 3)),
            (Vec3(-0.2, -1.0, -1.1), GlobalIndex(-1, -1, -2)),
        ];

        for (input, expected) in examples.into_iter() {
            let actual = GlobalIndex::from(input);
            assert_eq!(actual, expected);
        }
    }
}
