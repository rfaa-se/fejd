use crate::{
    components::logic::{Body, Motion},
    components::render::{RenderColor, RenderRectangle, RenderTriangle, RenderVector2, Renderable},
    entities::{Particle, Projectile, Triship},
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
                Flint::from_num(26),
                Flint::from_num(31),
            ),
            rotation: rotation.clone(),
        };

        let render = Renderable::<RenderTriangle>::new(
            RenderColor::GREEN,
            body.shape.into(),
            rotation.y.to_num::<f32>().atan2(rotation.x.to_num()),
        );

        let motion = Motion {
            speed: Flint::from_num(0),
            max_speed: Flint::from_num(8),
            acceleration: Flint::from_num(0.2),
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
        rotation: FlintVec2,
        render_centroid: RenderVector2,
        relative_speed: Flint,
        pid: usize,
    ) -> Projectile {
        let body = Body {
            shape: FlintRectangle::from_centroid(centroid, Flint::from_num(2), Flint::from_num(2)),
            rotation,
        };

        let mut rec: RenderRectangle = body.shape.into();
        rec.x = render_centroid.x;
        rec.y = render_centroid.y;

        let render = Renderable::<RenderRectangle>::new(
            RenderColor::GREEN,
            rec,
            body.rotation
                .y
                .to_num::<f32>()
                .atan2(body.rotation.x.to_num()),
        );

        let speed = Flint::from_num(14);

        let motion = Motion {
            // projectile will travel at base speed relative to entity that fired it,
            // if entity speed is 5 then the speed of the projectile will be 14 + 5
            speed: speed + relative_speed,
            max_speed: Flint::MAX,
            acceleration: speed,
            rotation_speed: Flint::ZERO,
        };

        Projectile {
            body,
            motion,
            render,
            dead: false,
            pid,
        }
    }

    pub fn spawn_particle(
        &self,
        centroid: &FlintVec2,
        render_centroid: RenderVector2,
        rotation: FlintVec2,
        speed: Flint,
        relative_speed: Flint,
        lifetime: i32,
    ) -> Particle {
        let body = Body {
            shape: centroid.clone(),
            rotation,
        };

        let motion = Motion {
            speed: speed + relative_speed,
            max_speed: Flint::MAX,
            acceleration: Flint::ZERO,
            rotation_speed: Flint::ZERO,
        };

        let render = Renderable::<RenderVector2>::new(
            RenderColor::GREEN,
            render_centroid,
            rotation.y.to_num::<f32>().atan2(rotation.x.to_num()),
        );

        Particle {
            body,
            motion,
            lifetime,
            render,
            dead: false,
        }
    }

    // pub fn _spawn_thruster_particles(
    //     &self,
    //     centroid: &FlintVec2,
    //     render_centroid: RenderVector2,
    //     rotation: FlintVec2,
    //     speed: Flint,
    //     relative_speed: Flint,
    // ) -> Vec<Particle> {
    //     let mut particles = Vec::new();

    //     // TODO

    //     particles
    // }
}
