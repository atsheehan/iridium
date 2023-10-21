use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Vec3(pub(crate) f32, pub(crate) f32, pub(crate) f32);

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
