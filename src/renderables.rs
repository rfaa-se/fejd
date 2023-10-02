use raylib::prelude::{Color, RaylibDraw, RaylibMode2D, Vector2};

use crate::{components::Body, math::FlintTriangle, misc::RaylibRenderHandle};

pub struct Renderable<T> {
    pub color: Color,
    pub live: RenderBody<T>,
    pub past: RenderBody<T>,
}

#[derive(Clone, Copy)]
pub struct RenderBody<T> {
    pub shape: T,
    pub rotation: f32,
}

#[derive(Clone, Copy)]
pub struct RenderTriangle {
    pub v1: Vector2,
    pub v2: Vector2,
    pub v3: Vector2,
}

impl From<FlintTriangle> for RenderTriangle {
    fn from(value: FlintTriangle) -> Self {
        Self {
            v1: Vector2 {
                x: value.v1.x.to_num(),
                y: value.v1.y.to_num(),
            },
            v2: Vector2 {
                x: value.v2.x.to_num(),
                y: value.v2.y.to_num(),
            },
            v3: Vector2 {
                x: value.v3.x.to_num(),
                y: value.v3.y.to_num(),
            },
        }
    }
}

impl From<Body<FlintTriangle>> for RenderBody<RenderTriangle> {
    fn from(value: Body<FlintTriangle>) -> Self {
        // transform the rotation vector into radians
        let rotation = value
            .rotation
            .y
            .to_num::<f32>()
            .atan2(value.rotation.x.to_num());

        let mut shape: RenderTriangle = value.shape.into();
        shape.rotate(rotation);

        Self { shape, rotation }
    }
}

impl Renderable<RenderTriangle> {
    pub fn new(color: Color, shape: &RenderTriangle, rotation: f32) -> Self {
        Renderable {
            color,
            live: RenderBody {
                shape: shape.clone(),
                rotation,
            },
            past: RenderBody {
                shape: shape.clone(),
                rotation,
            },
        }
    }

    pub fn lerp_v1(&self, amount: f32) -> Vector2 {
        self.past.shape.v1.lerp(self.live.shape.v1, amount)
    }

    pub fn lerp_v2(&self, amount: f32) -> Vector2 {
        self.past.shape.v2.lerp(self.live.shape.v2, amount)
    }

    pub fn lerp_v3(&self, amount: f32) -> Vector2 {
        self.past.shape.v3.lerp(self.live.shape.v3, amount)
    }

    pub fn lerp_centroid(&self, amount: f32) -> Vector2 {
        self.past
            .shape
            .get_centroid()
            .lerp(self.live.shape.get_centroid(), amount)
    }

    pub fn draw(&self, rrh: &mut RaylibMode2D<RaylibRenderHandle>, delta: f32) {
        rrh.draw_triangle_lines(
            self.lerp_v1(delta),
            self.lerp_v2(delta),
            self.lerp_v3(delta),
            self.color,
        );
    }
}

impl RenderTriangle {
    pub fn get_centroid(&self) -> Vector2 {
        Vector2 {
            x: (self.v1.x + self.v2.x + self.v3.x) / 3.0,
            y: (self.v1.y + self.v2.y + self.v3.y) / 3.0,
        }
    }

    pub fn rotate(&mut self, amount: f32) {
        let cen = self.get_centroid();
        let cos = amount.cos();
        let sin = amount.sin();
        let rot = |v: &mut Vector2| {
            let x = (cos * (v.x - cen.x)) - (sin * (v.y - cen.y)) + cen.x;
            let y = (sin * (v.x - cen.x)) + (cos * (v.y - cen.y)) + cen.y;
            v.x = x;
            v.y = y;
        };

        // rotate all three vectors
        rot(&mut self.v1);
        rot(&mut self.v2);
        rot(&mut self.v3);
    }
}
