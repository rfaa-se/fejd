use fixed::types::I20F12;

pub type Flint = I20F12;

#[derive(Clone, Copy, Debug)]
pub struct FlintVec2 {
    pub x: Flint,
    pub y: Flint,
}

#[derive(Clone, Copy, Debug)]
pub struct FlintTriangle {
    pub v1: FlintVec2,
    pub v2: FlintVec2,
    pub v3: FlintVec2,
    pub width: Flint,
    pub height: Flint,
}

impl FlintVec2 {
    pub fn new(x: Flint, y: Flint) -> Self {
        FlintVec2 { x, y }
    }

    pub fn _radians(&self) -> Flint {
        cordic::atan2(self.y, self.x)
    }
}

impl FlintTriangle {
    pub fn from_centroid(cen: &FlintVec2, width: Flint, height: Flint) -> Self {
        // bottom left
        let v1 = FlintVec2::new(
            Flint::from_num(cen.x - (width / 2)),
            Flint::from_num(cen.y + (height / 3)),
        );

        // top middle
        let v2 = FlintVec2::new(
            Flint::from_num(cen.x),
            Flint::from_num(cen.y - ((height / 3) * 2)),
        );

        // bottom right
        let v3 = FlintVec2::new(
            Flint::from_num(cen.x + (width / 2)),
            Flint::from_num(cen.y + (height / 3)),
        );

        FlintTriangle {
            v1,
            v2,
            v3,
            width,
            height,
        }
    }

    pub fn _get_centroid(&self) -> FlintVec2 {
        FlintVec2 {
            x: ((self.v1.x + self.v2.x + self.v3.x) / 3),
            y: ((self.v1.y + self.v2.y + self.v3.y) / 3),
        }
    }
}
