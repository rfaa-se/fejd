use std::ops::{Add, AddAssign, Mul, Sub};

use fixed::types::I20F12;

pub type Flint = I20F12;

#[derive(Clone, Copy, Debug)]
pub struct FlintVec2 {
    pub x: Flint,
    pub y: Flint,
}

#[derive(Clone, Copy, Debug)]
pub struct FlintLine {
    pub v1: FlintVec2,
    pub v2: FlintVec2,
}

#[derive(Clone, Copy, Debug)]
pub struct FlintTriangle {
    pub v1: FlintVec2,
    pub v2: FlintVec2,
    pub v3: FlintVec2,
    pub width: Flint,
    pub height: Flint,
}

#[derive(Clone, Copy, Debug)]
pub struct FlintRectangle {
    pub point: FlintVec2,
    pub width: Flint,
    pub height: Flint,
}

impl FlintRectangle {
    pub fn from_centroid(cen: &FlintVec2, width: Flint, height: Flint) -> Self {
        Self {
            point: FlintVec2::new(cen.x - width / 2, cen.y - height / 2),
            width,
            height,
        }
    }

    pub fn get_centroid(&self) -> FlintVec2 {
        FlintVec2 {
            x: self.point.x + self.width / 2,
            y: self.point.y + self.height / 2,
        }
    }
}

impl FlintVec2 {
    pub fn new(x: Flint, y: Flint) -> Self {
        FlintVec2 { x, y }
    }

    pub fn radians(&self) -> Flint {
        cordic::atan2(self.y, self.x)
    }

    pub const fn rotation_north() -> FlintVec2 {
        FlintVec2 {
            x: Flint::ZERO,
            y: Flint::NEG_ONE,
        }
    }

    pub const fn rotation_east() -> FlintVec2 {
        FlintVec2 {
            x: Flint::ONE,
            y: Flint::ZERO,
        }
    }

    pub const fn rotation_south() -> FlintVec2 {
        FlintVec2 {
            x: Flint::ZERO,
            y: Flint::ONE,
        }
    }

    pub const fn rotation_west() -> FlintVec2 {
        FlintVec2 {
            x: Flint::NEG_ONE,
            y: Flint::ZERO,
        }
    }

    pub fn rotate(&self, rad: &Flint, around: &FlintVec2) -> FlintVec2 {
        let cos = cordic::cos(*rad);
        let sin = cordic::sin(*rad);
        let x = self.x - around.x;
        let y = self.y - around.y;

        FlintVec2 {
            x: (cos * x) - (sin * y) + around.x,
            y: (sin * x) + (cos * y) + around.y,
        }
    }

    pub fn rotate_180(&self) -> FlintVec2 {
        FlintVec2 {
            x: self.x * -1,
            y: self.y * -1,
        }
    }

    pub fn rotate_90(&self) -> FlintVec2 {
        FlintVec2 {
            x: self.y,
            y: self.x * -1,
        }
    }

    pub fn rotate_270(&self) -> FlintVec2 {
        FlintVec2 {
            x: self.y * -1,
            y: self.x,
        }
    }

    pub fn normalize(&self) -> FlintVec2 {
        let mag = self.magnitude();

        if mag == Flint::ZERO {
            return FlintVec2 {
                x: Flint::ZERO,
                y: Flint::ZERO,
            };
        }

        FlintVec2 {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    pub fn perpendicular(&self) -> FlintVec2 {
        FlintVec2 {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn magnitude(&self) -> Flint {
        cordic::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn dot(&self, other: &FlintVec2) -> Flint {
        Flint::from_num(self.x * other.x + self.y * other.y)
    }
}

impl FlintTriangle {
    pub fn from_centroid(cen: &FlintVec2, width: Flint, height: Flint) -> Self {
        //          up 90
        // left 0            right 180
        //         down 270
        //
        // v1
        // |\
        // | \ v2
        // | /
        // |/
        // v3

        let left = FlintVec2::new(
            Flint::from_num(cen.x - (width / 2)),
            Flint::from_num(cen.y - (height / 3)),
        );

        let top = FlintVec2::new(Flint::from_num(cen.x + width / 2), Flint::from_num(cen.y));

        let right = FlintVec2::new(
            Flint::from_num(cen.x - (width / 2)),
            Flint::from_num(cen.y + (height / 3)),
        );

        FlintTriangle {
            v1: left,
            v2: top,
            v3: right,
            width,
            height,
        }
    }

    pub fn get_centroid(&self) -> FlintVec2 {
        FlintVec2 {
            x: ((self.v1.x + self.v2.x + self.v3.x) / 3),
            y: ((self.v1.y + self.v2.y + self.v3.y) / 3),
        }
    }
}

impl Mul<Flint> for FlintVec2 {
    type Output = FlintVec2;

    fn mul(self, rhs: Flint) -> Self::Output {
        FlintVec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<FlintVec2> for FlintVec2 {
    type Output = FlintVec2;

    fn mul(self, rhs: FlintVec2) -> Self::Output {
        FlintVec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl AddAssign for FlintVec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for FlintVec2 {
    type Output = FlintVec2;

    fn add(self, rhs: Self) -> Self::Output {
        FlintVec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for FlintVec2 {
    type Output = FlintVec2;

    fn sub(self, rhs: Self) -> Self::Output {
        FlintVec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
