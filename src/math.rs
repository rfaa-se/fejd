use fixed::types::I20F12;
use raylib::prelude::{lerp, Vector2};

pub type Flint = I20F12;

#[derive(Clone, Copy, Debug)]
pub struct FlintVec2 {
    pub x: Flint,
    pub y: Flint,
}

#[derive(Clone, Copy)]
pub struct FlintTriangle {
    pub v1: FlintVec2,
    pub v2: FlintVec2,
    pub v3: FlintVec2,
}

impl FlintVec2 {
    pub fn new(x: Flint, y: Flint) -> Self {
        FlintVec2 { x, y }
    }

    pub fn lerp(&self, other: &FlintVec2, amount: f32) -> Vector2 {
        // not to be used in game logic, only for rendering
        let v1 = Into::<Vector2>::into(*self);
        let v2 = Into::<Vector2>::into(*other);

        Vector2::new(lerp(v1.x, v2.x, amount), lerp(v1.y, v2.y, amount))
    }
}

impl From<FlintVec2> for Vector2 {
    fn from(val: FlintVec2) -> Self {
        Vector2::new(val.x.to_num(), val.y.to_num())
    }
}

impl FlintTriangle {
    pub fn from_center(
        center: FlintVec2,
        width: Flint,
        height: Flint,
        rotation: FlintVec2,
    ) -> Self {
        // bottom left
        let v1 = FlintVec2::new(
            Flint::from_num(center.x - (width / 2)),
            Flint::from_num(center.y + (height / 2)),
        );

        // top middle
        let v2 = FlintVec2::new(
            Flint::from_num(center.x),
            Flint::from_num(center.y - (height / 2)),
        );

        // bottom right
        let v3 = FlintVec2::new(
            Flint::from_num(center.x + (width / 2)),
            Flint::from_num(center.y + (height / 2)),
        );

        FlintTriangle { v1, v2, v3 }.set_rotation(rotation)
    }

    pub fn calc_center(&self) -> FlintVec2 {
        // TODO: probably need to rotate to make math work in case of rotation?
        FlintVec2 {
            x: self.v1.x + ((self.v3.x - self.v1.x) / 2).round(),
            y: self.v2.y + ((self.v1.y - self.v2.y) / 2).round(),
        }
    }

    pub fn set_rotation(self, _rotation: FlintVec2) -> Self {
        self
    }
}
