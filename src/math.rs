use std::ops::{Add, Div, Sub};

#[derive(Debug, Copy, Clone)]
pub(crate) struct Vec2(pub(crate) f32, pub(crate) f32);

impl Vec2 {
    pub(crate) fn from_angle(angle: f32) -> Self {
        Self(angle.cos(), angle.sin())
    }

    pub(crate) fn dot(&self, rhs: &Vec2) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, denominator: f32) -> Self::Output {
        Self(self.0 / denominator, self.1 / denominator)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Vec3(pub(crate) f32, pub(crate) f32, pub(crate) f32);

impl Vec3 {
    pub(crate) fn set_x(&self, new_x: f32) -> Vec3 {
        Self(new_x, self.1, self.2)
    }

    pub(crate) fn set_y(&self, new_y: f32) -> Vec3 {
        Self(self.0, new_y, self.2)
    }

    pub(crate) fn set_z(&self, new_z: f32) -> Vec3 {
        Self(self.0, self.1, new_z)
    }

    pub(crate) fn rotate_y(&self, angle: f32) -> Vec3 {
        let Self(x, y, z) = self;

        let cos = angle.cos();
        let sin = angle.sin();

        let new_x = x * cos + z * sin;
        let new_y = *y;
        let new_z = x * -sin + z * cos;

        Self(new_x, new_y, new_z)
    }

    pub(crate) fn x(&self) -> f32 {
        self.0
    }

    pub(crate) fn y(&self) -> f32 {
        self.1
    }

    pub(crate) fn z(&self) -> f32 {
        self.2
    }

    pub(crate) fn xz(&self) -> Vec2 {
        Vec2(self.0, self.2)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

pub(crate) struct RandomNumberGenerator {
    seed: u32,
}

impl RandomNumberGenerator {
    pub(crate) fn with_seed(seed: u32) -> Self {
        Self { seed }
    }

    pub(crate) fn gen_u32(&mut self) -> u32 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 17;
        self.seed ^= self.seed << 5;
        self.seed
    }

    pub(crate) fn gen_range(&mut self, min: u32, max: u32) -> u32 {
        let range = max - min;
        min + self.gen_u32() % range
    }

    // Generates an f32 value between [0.0, 1.0).
    pub(crate) fn gen_f32(&mut self) -> f32 {
        let value = self.gen_u32();
        value as f32 / u32::MAX as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::*;

    #[test]
    fn rotate_vector_around_y_axis() {
        let examples = [
            (Vec3(1.0, 0.0, 0.0), PI, Vec3(-1.0, 0.0, 0.0)),
            (Vec3(-1.0, 0.0, 0.0), PI, Vec3(1.0, 0.0, 0.0)),
            (Vec3(0.5, 0.0, 0.0), FRAC_PI_2, Vec3(0.0, 0.0, -0.5)),
            (Vec3(0.5, 0.0, 0.0), -FRAC_PI_2, Vec3(0.0, 0.0, 0.5)),
            (Vec3(0.0, 1.0, 0.0), FRAC_PI_2, Vec3(0.0, 1.0, 0.0)),
            (
                Vec3(0.0, 0.0, 1.0),
                FRAC_PI_4,
                Vec3(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2),
            ),
        ];

        for (input, angle, expected_output) in examples.into_iter() {
            let actual_output = input.rotate_y(angle);
            assert_vec3s_equal(&actual_output, &expected_output);
        }
    }

    fn assert_vec3s_equal(a: &Vec3, b: &Vec3) {
        const TOLERANCE: f32 = 0.00001;
        let vecs_equal = (a.0 - b.0).abs() < TOLERANCE
            && (a.1 - b.1).abs() < TOLERANCE
            && (a.2 - b.2).abs() < TOLERANCE;
        assert!(
            vecs_equal,
            "vecs are not equal:\n  left: {:?}\n right: {:?}\n",
            a, b
        );
    }

    #[test]
    fn check_distribution_of_random_f32s() {
        const NUM_EXAMPLES: u32 = 10_000;
        const NUM_BUCKETS: u32 = 10;
        const EXPECTED_BUCKET_COUNT: u32 = NUM_EXAMPLES / NUM_BUCKETS;
        const MAX_ALLOWED_DEVIATION: u32 = 100;

        let mut buckets = [0; NUM_BUCKETS as usize];
        let mut rng = RandomNumberGenerator::with_seed(314159);

        for _ in 0..NUM_EXAMPLES {
            let value = rng.gen_f32();
            let i = (value / 0.1) as usize;
            buckets[i] += 1;
        }

        for count in buckets.iter() {
            assert!((*count - EXPECTED_BUCKET_COUNT as i32).unsigned_abs() < MAX_ALLOWED_DEVIATION);
        }
    }

    #[test]
    fn create_vec2_from_angle() {
        let examples = [
            (0.0, Vec2(1.0, 0.0)),
            (PI, Vec2(-1.0, 0.0)),
            (2.0 * PI, Vec2(1.0, 0.0)),
            (FRAC_PI_2, Vec2(0.0, 1.0)),
            (FRAC_PI_3, Vec2(0.5, 0.866025)),
        ];

        for (angle, expected) in examples.into_iter() {
            let actual = Vec2::from_angle(angle);
            assert_vec2s_equal(&actual, &expected);
        }
    }

    fn assert_vec2s_equal(a: &Vec2, b: &Vec2) {
        const TOLERANCE: f32 = 0.00001;
        let vecs_equal = (a.0 - b.0).abs() < TOLERANCE && (a.1 - b.1).abs() < TOLERANCE;
        assert!(
            vecs_equal,
            "vecs are not equal:\n  left: {:?}\n right: {:?}\n",
            a, b
        );
    }
}

pub(crate) fn interpolate(value_a: f32, value_b: f32, t: f32) -> f32 {
    (1.0 - t) * value_a + t * value_b
}
