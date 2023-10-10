use raylib::prelude::{Color, Vector2};

use crate::{
    components::logic::{Body, Motion},
    components::render::{RenderRectangle, RenderTriangle, Renderable},
    entities::{Projectile, Triship},
    math::{Flint, FlintRectangle, FlintTriangle, FlintVec2},
};

pub struct Spawner;

impl Spawner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn spawn_triship(&self, centroid: &FlintVec2, rotation: &FlintVec2) -> Triship {
        let body = Body {
            shape: FlintTriangle::from_centroid(
                &centroid,
                Flint::from_num(27),
                Flint::from_num(31),
            ),
            rotation: rotation.clone(),
        };

        let render = Renderable::<RenderTriangle>::new(
            Color::GREEN,
            &body.shape.into(),
            rotation.y.to_num::<f32>().atan2(rotation.x.to_num()),
        );

        let motion = Motion {
            speed: Flint::from_num(0),
            max_speed: Flint::from_num(12),
            acceleration: Flint::from_num(0.6),
            rotation_speed: Flint::from_num(0.18),
        };

        Triship {
            body,
            motion,
            render,
            dead: false,
        }
    }

    pub fn spawn_projectile(
        &self,
        centroid: &FlintVec2,
        rotation: &FlintVec2,
        render_centroid: &Vector2,
        render_rotation: f32,
        rel_speed: &Flint,
    ) -> Projectile {
        let body = Body {
            shape: FlintRectangle::from_centroid(centroid, Flint::from_num(2), Flint::from_num(2)),
            rotation: rotation.clone(),
        };

        let mut rec: RenderRectangle = body.shape.into();
        rec.point = *render_centroid;

        let render = Renderable::<RenderRectangle>::new(
            Color::GREEN,
            &rec,
            render_rotation, // body.rotation
                             //     .y
                             //     .to_num::<f32>()
                             //     .atan2(body.rotation.x.to_num()),
        );

        let speed = Flint::from_num(14);

        let motion = Motion {
            // projectile will travel at base speed relative to entity that fired it,
            // if entity speed is 5 then the speed of the projectile will be 14 + 5
            speed: speed + rel_speed,
            max_speed: Flint::MAX,
            acceleration: speed,
            rotation_speed: Flint::ZERO,
        };

        Projectile {
            body,
            motion,
            render,
            dead: false,
        }
    }
}
