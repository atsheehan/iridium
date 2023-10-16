pub(crate) struct Vec3(pub(crate) f32, pub(crate) f32, pub(crate) f32);

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
