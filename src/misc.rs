use raylib::{prelude::RaylibTextureMode, RaylibHandle};

// just a convenience type so we can type
// RaylibRenderHandle instead of
// RaylibTextureMode<&mut RaylibHandle>
pub type RaylibRenderHandle<'a> = RaylibTextureMode<'a, &'a mut RaylibHandle>;
