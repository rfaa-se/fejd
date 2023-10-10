use raylib::{
    prelude::{RaylibTextureMode, Vector2},
    RaylibHandle,
};

// just a convenience type so we can type
// RaylibRenderHandle instead of
// RaylibTextureMode<&mut RaylibHandle>
pub type RaylibRenderHandle<'a> = RaylibTextureMode<'a, &'a mut RaylibHandle>;

pub fn rotate_vector2(point: &Vector2, rad: &f32, around: &Vector2) -> Vector2 {
    let (sin, cos) = rad.sin_cos();
    let x = point.x - around.x;
    let y = point.y - around.y;

    Vector2 {
        x: (cos * x) - (sin * y) + around.x,
        y: (sin * x) + (cos * y) + around.y,
    }
}
