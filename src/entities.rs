use raylib::prelude::*;

use crate::math::FlintTriangle;

pub struct Player {
    pub color: Color,
    pub position: Position<FlintTriangle>,
}

#[derive(Clone, Copy)]
pub struct Position<T> {
    pub current: T,
    pub old: T,
}

impl Position<FlintTriangle> {
    pub fn new(pos: FlintTriangle) -> Self {
        Position {
            current: pos,
            old: pos,
        }
    }

    pub fn lerp_v1(&self, amount: f32) -> Vector2 {
        self.current.v1.lerp(&self.old.v1, amount)
    }

    pub fn lerp_v2(&self, amount: f32) -> Vector2 {
        self.current.v2.lerp(&self.old.v2, amount)
    }

    pub fn lerp_v3(&self, amount: f32) -> Vector2 {
        self.current.v3.lerp(&self.old.v3, amount)
    }

    pub fn lerp_center(&self, amount: f32) -> Vector2 {
        self.current
            .calc_center()
            .lerp(&self.old.calc_center(), amount)
    }
}
