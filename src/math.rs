use std::ops::Add;

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
}
