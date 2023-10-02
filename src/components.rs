use crate::math::FlintVec2;

#[derive(Clone, Copy)]
pub struct Body<T> {
    pub shape: T,
    pub rotation: FlintVec2,
}
