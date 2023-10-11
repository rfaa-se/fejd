use raylib::prelude::{Color, Rectangle, Vector2};

use crate::components::logic::Body;
use crate::math::{FlintRectangle, FlintTriangle, FlintVec2};

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

#[derive(Clone, Copy)]
pub struct RenderRectangle {
    pub point: Vector2,
    pub width: f32,
    pub height: f32,
}

impl From<FlintVec2> for Vector2 {
    fn from(value: FlintVec2) -> Self {
        Self {
            x: value.x.to_num(),
            y: value.y.to_num(),
        }
    }
}

impl From<Body<FlintVec2>> for RenderBody<Vector2> {
    fn from(value: Body<FlintVec2>) -> Self {
        Self {
            shape: value.shape.into(),
            rotation: value
                .rotation
                .y
                .to_num::<f32>()
                .atan2(value.rotation.x.to_num()),
        }
    }
}

impl From<FlintRectangle> for RenderRectangle {
    fn from(value: FlintRectangle) -> Self {
        Self {
            point: Vector2 {
                x: value.point.x.to_num::<f32>(),
                y: value.point.y.to_num::<f32>(),
            },
            width: value.width.to_num::<f32>(),
            height: value.height.to_num::<f32>(),
        }
    }
}

impl From<Body<FlintRectangle>> for RenderBody<RenderRectangle> {
    fn from(value: Body<FlintRectangle>) -> Self {
        // transform the rotation vector into radians
        let rotation = value
            .rotation
            .y
            .to_num::<f32>()
            .atan2(value.rotation.x.to_num());

        let shape: RenderRectangle = value.shape.into();

        Self { shape, rotation }
    }
}

impl From<FlintTriangle> for RenderTriangle {
    fn from(value: FlintTriangle) -> Self {
        Self {
            v1: Vector2 {
                x: value.v1.x.to_num::<f32>(),
                y: value.v1.y.to_num::<f32>(),
            },
            v2: Vector2 {
                x: value.v2.x.to_num::<f32>(),
                y: value.v2.y.to_num::<f32>(),
            },
            v3: Vector2 {
                x: value.v3.x.to_num::<f32>(),
                y: value.v3.y.to_num::<f32>(),
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

impl<T> Renderable<T> {
    pub fn lerp_rotation(&self, amount: f32) -> f32 {
        raylib::math::lerp(self.past.rotation, self.live.rotation, amount)
    }
}

impl Renderable<RenderTriangle> {
    pub fn new(color: Color, mut shape: RenderTriangle, rotation: f32) -> Self {
        shape.rotate(rotation);

        Renderable {
            color,
            live: RenderBody { shape, rotation },
            past: RenderBody { shape, rotation },
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
        let (sin, cos) = amount.sin_cos();
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

impl Renderable<RenderRectangle> {
    pub fn new(color: Color, shape: RenderRectangle, rotation: f32) -> Self {
        Renderable {
            color,
            live: RenderBody { shape, rotation },
            past: RenderBody { shape, rotation },
        }
    }

    pub fn lerp(&self, amount: f32) -> Rectangle {
        // we return a raylib rectangle from this instead of a render rectangle,
        // this is so we can easily just use the raylib draw methods without further
        // conversions
        let point = self.past.shape.point.lerp(self.live.shape.point, amount);

        Rectangle {
            x: point.x,
            y: point.y,
            width: raylib::math::lerp(self.past.shape.width, self.live.shape.width, amount),
            height: raylib::math::lerp(self.past.shape.height, self.live.shape.height, amount),
        }
    }
}
