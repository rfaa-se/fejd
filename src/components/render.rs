use raylib::prelude::{Color, Rectangle, Vector2};

use crate::components::logic::Body;
use crate::math::{FlintRectangle, FlintTriangle, FlintVec2};

pub struct Renderable<T> {
    pub color: RenderColor,
    pub live: RenderBody<T>,
    pub past: RenderBody<T>,
}

#[derive(Clone, Copy)]
pub struct RenderBody<T> {
    pub shape: T,
    pub angle: f32,
}

// let's reuse what's in raylib
pub type RenderColor = Color;
pub type RenderVector2 = Vector2;
pub type RenderRectangle = Rectangle;

#[derive(Clone, Copy)]
pub struct RenderTriangle {
    pub v1: RenderVector2,
    pub v2: RenderVector2,
    pub v3: RenderVector2,
}

impl<T> Renderable<T> {
    pub fn lerp_angle(&self, amount: f32) -> f32 {
        raylib::math::lerp(self.past.angle, self.live.angle, amount)
    }
}

impl Renderable<RenderVector2> {
    pub fn new(color: RenderColor, shape: RenderVector2, angle: f32) -> Self {
        Renderable {
            color,
            live: RenderBody { shape, angle },
            past: RenderBody { shape, angle },
        }
    }

    pub fn lerp(&self, amount: f32) -> RenderVector2 {
        self.past.shape.lerp(self.live.shape, amount)
    }
}

impl From<FlintVec2> for RenderVector2 {
    fn from(value: FlintVec2) -> Self {
        Self {
            x: value.x.to_num(),
            y: value.y.to_num(),
        }
    }
}

impl From<&Body<FlintVec2>> for RenderBody<RenderVector2> {
    fn from(value: &Body<FlintVec2>) -> Self {
        Self {
            shape: value.live.shape.into(),
            angle: value.live.direction.radians().to_num::<f32>(),
        }
    }
}

impl RenderTriangle {
    pub fn centroid(&self) -> Vector2 {
        Vector2 {
            x: (self.v1.x + self.v2.x + self.v3.x) / 3.0,
            y: (self.v1.y + self.v2.y + self.v3.y) / 3.0,
        }
    }

    fn rotate(&mut self, amount: f32) {
        let cen = self.centroid();
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

impl Renderable<RenderTriangle> {
    pub fn new(color: RenderColor, mut shape: RenderTriangle, angle: f32) -> Self {
        shape.rotate(angle);

        Renderable {
            color,
            live: RenderBody { shape, angle },
            past: RenderBody { shape, angle },
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
            .centroid()
            .lerp(self.live.shape.centroid(), amount)
    }

    pub fn lerp(&self, amount: f32) -> RenderTriangle {
        RenderTriangle {
            v1: self.lerp_v1(amount),
            v2: self.lerp_v2(amount),
            v3: self.lerp_v3(amount),
        }
    }
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

impl From<&Body<FlintTriangle>> for RenderBody<RenderTriangle> {
    fn from(value: &Body<FlintTriangle>) -> Self {
        let angle = value.live.direction.radians().to_num();
        let mut shape: RenderTriangle = value.live.shape.into();
        shape.rotate(angle);

        Self { shape, angle }
    }
}

impl Renderable<RenderRectangle> {
    pub fn new(color: RenderColor, shape: RenderRectangle, angle: f32) -> Self {
        Renderable {
            color,
            live: RenderBody { shape, angle },
            past: RenderBody { shape, angle },
        }
    }

    pub fn lerp(&self, amount: f32) -> RenderRectangle {
        Rectangle {
            x: raylib::math::lerp(self.past.shape.x, self.live.shape.x, amount),
            y: raylib::math::lerp(self.past.shape.y, self.live.shape.y, amount),
            width: raylib::math::lerp(self.past.shape.width, self.live.shape.width, amount),
            height: raylib::math::lerp(self.past.shape.height, self.live.shape.height, amount),
        }
    }
}

impl From<FlintRectangle> for RenderRectangle {
    fn from(value: FlintRectangle) -> Self {
        Self {
            x: value.point.x.to_num(),
            y: value.point.y.to_num(),
            width: value.width.to_num(),
            height: value.height.to_num(),
        }
    }
}

impl From<&Body<FlintRectangle>> for RenderBody<RenderRectangle> {
    fn from(value: &Body<FlintRectangle>) -> Self {
        let angle = value.live.direction.radians().to_num::<f32>();
        let shape: RenderRectangle = value.live.shape.into();

        Self { shape, angle }
    }
}
