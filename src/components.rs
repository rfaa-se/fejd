use crate::math::{Flint, FlintVec2};

#[derive(Clone, Copy)]
pub struct Body<T> {
    pub shape: T,
    pub rotation: FlintVec2,
}

pub struct Motion {
    pub speed: Flint,
    pub max_speed: Flint,
    pub acceleration: Flint,
    pub rotation_speed: Flint,
}
