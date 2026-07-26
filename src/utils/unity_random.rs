/// Bộ mô phỏng PRNG gốc của Unity Engine (UnityEngine.Random - Xorshift128)
#[derive(Debug, Clone, Copy)]
pub struct UnityRandom {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl UnityRandom {
    pub fn new(seed: i32) -> Self {
        let mut rng = Self {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        };
        rng.init_state(seed);
        rng
    }

    pub fn init_state(&mut self, seed: i32) {
        let s = seed as u32;
        self.x = if s == 0 { 1 } else { s };
        self.y = self.x.wrapping_mul(1812433253).wrapping_add(1);
        self.z = self.y.wrapping_mul(1812433253).wrapping_add(1);
        self.w = self.z.wrapping_mul(1812433253).wrapping_add(1);
    }

    pub fn next_u32(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
        self.w
    }

    pub fn value(&mut self) -> f32 {
        // Đã cập nhật công thức chuyển đổi Float chuẩn theo chiều thuận của Unity C++ Native Engine
        let raw = self.next_u32() & 0x007FFFFF;
        let float_val = (raw as f32) / 8388607.0;
        if float_val > 0.5 {
            1.0 - float_val
        } else {
            float_val
        }
    }

    pub fn range_float(&mut self, min: f32, max: f32) -> f32 {
        min + self.value() * (max - min)
    }

    pub fn range_int(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            return min;
        }
        let diff = (max - min) as u64;
        let r = (self.next_u32() as u64) % diff;
        min + r as i32
    }
}
