use std::ops::{Add, Sub};

/// Axial Hex Coordinate (q, r). Cube representation is (q, r, -q - r).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AxialPos {
    pub q: i32,
    pub r: i32,
}

impl AxialPos {
    pub const ZERO: Self = AxialPos { q: 0, r: 0 };

    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    /// 6 Neighbor directions starting from Top-Right (Direction 0) clockwise:
    /// 0: (+1, -1) [Top-Right]
    /// 1: (+1, 0)  [Right]
    /// 2: (0, +1)  [Bottom-Right]
    /// 3: (-1, +1) [Bottom-Left]
    /// 4: (-1, 0)  [Left]
    /// 5: (0, -1)  [Top-Left]
    pub const DIRECTIONS: [AxialPos; 6] = [
        AxialPos { q: 1, r: -1 },
        AxialPos { q: 1, r: 0 },
        AxialPos { q: 0, r: 1 },
        AxialPos { q: -1, r: 1 },
        AxialPos { q: -1, r: 0 },
        AxialPos { q: 0, r: -1 },
    ];

    pub fn neighbor(self, dir: usize) -> Self {
        self + Self::DIRECTIONS[dir % 6]
    }

    pub fn opposite_dir(dir: usize) -> usize {
        (dir + 3) % 6
    }

    pub fn distance(self, other: Self) -> i32 {
        let dq = (self.q - other.q).abs();
        let dr = (self.r - other.r).abs();
        let ds = ((-self.q - self.r) - (-other.q - other.r)).abs();
        (dq + dr + ds) / 2
    }

    /// Converts axial hex coords to 2D pixel coords (Pointy-topped hexes).
    /// hex_radius is the distance from center to vertex.
    pub fn to_pixel(self, hex_radius: f32) -> (f32, f32) {
        let x = hex_radius * (3.0f32.sqrt() * self.q as f32 + 3.0f32.sqrt() / 2.0 * self.r as f32);
        let y = hex_radius * (3.0 / 2.0 * self.r as f32);
        (x, y)
    }

    /// Converts 2D pixel coords back to nearest axial hex coords.
    pub fn from_pixel(x: f32, y: f32, hex_radius: f32) -> Self {
        let q = (3.0f32.sqrt() / 3.0 * x - 1.0 / 3.0 * y) / hex_radius;
        let r = (2.0 / 3.0 * y) / hex_radius;
        Self::round(q, r)
    }

    fn round(q: f32, r: f32) -> Self {
        let s = -q - r;
        let mut rq = q.round();
        let mut rr = r.round();
        let rs = s.round();

        let q_diff = (rq - q).abs();
        let r_diff = (rr - r).abs();
        let s_diff = (rs - s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            rq = -rr - rs;
        } else if r_diff > s_diff {
            rr = -rq - rs;
        }

        Self {
            q: rq as i32,
            r: rr as i32,
        }
    }
}

impl Add for AxialPos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl Sub for AxialPos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}
