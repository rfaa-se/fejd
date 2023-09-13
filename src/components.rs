use raylib::prelude::Vector2;

use crate::math::{FlintTriangle, FlintVec2};

#[derive(Clone, Copy)]
pub struct Body<T> {
    pub current: T,
    pub old: T,
    pub rotation: FlintVec2,
}

impl Body<FlintTriangle> {
    pub fn new(pos: FlintTriangle, rot: FlintVec2) -> Self {
        pos.set_rotation(rot);
        Body {
            current: pos,
            old: pos,
            rotation: rot,
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
            .get_centroid()
            .lerp(&self.old.get_centroid(), amount)
    }
}
