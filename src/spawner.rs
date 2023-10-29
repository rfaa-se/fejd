use fastrand::Rng;

use crate::{
    components::logic::{Body, Motion},
    components::{
        logic::Shape,
        render::{RenderColor, RenderRectangle, RenderTriangle, RenderVector2, Renderable},
    },
    entities::{Particle, Projectile, Star, Triship},
    math::{Flint, FlintRectangle, FlintTriangle, FlintVec2},
};

pub struct Spawner;

impl Spawner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn spawn_triship(&self, centroid: &FlintVec2, rotation: &FlintVec2) -> Triship {
        let shape = Shape {
            shape: FlintTriangle::from_centroid(
                &centroid,
                Flint::from_num(26),
                Flint::from_num(31),
            ),
            rotation: rotation.clone(),
        };
        let body = Body {
            live: shape,
            past: shape,
            dirty: true,
            axes: Vec::new(),
        };

        let render = Renderable::<RenderTriangle>::new(
            RenderColor::DIMGRAY,
            body.live.shape.into(),
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
            life: Flint::from_num(50),
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
        let shape = Shape {
            shape: FlintRectangle::from_centroid(centroid, Flint::from_num(2), Flint::from_num(1)),
            rotation,
        };

        let body = Body {
            live: shape,
            past: shape,
            dirty: true,
            axes: Vec::new(),
        };

        let mut rec: RenderRectangle = body.live.shape.into();
        rec.x = render_centroid.x;
        rec.y = render_centroid.y;

        let render = Renderable::<RenderRectangle>::new(
            RenderColor::LIGHTYELLOW,
            rec,
            body.live
                .rotation
                .y
                .to_num::<f32>()
                .atan2(body.live.rotation.x.to_num()),
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
            dmg: Flint::from_num(1),
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
        color: RenderColor,
        amount: u8, // TODO: naming...
    ) -> Particle {
        let shape = Shape {
            shape: centroid.clone(),
            rotation,
        };

        let body = Body {
            live: shape,
            past: shape,
            dirty: true,
            axes: Vec::new(),
        };

        let motion = Motion {
            speed: speed + relative_speed,
            max_speed: Flint::MAX,
            acceleration: Flint::ZERO,
            rotation_speed: Flint::ZERO,
        };

        let render = Renderable::<RenderVector2>::new(
            color,
            render_centroid,
            rotation.y.to_num::<f32>().atan2(rotation.x.to_num()),
        );

        Particle {
            body,
            motion,
            lifetime,
            render,
            dead: false,
            amount,
        }
    }

    pub fn spawn_thruster_particles(
        &self,
        centroid: &FlintVec2,
        render_centroid: RenderVector2,
        rotation: FlintVec2,
        min_speed: Flint,
        relative_speed: Flint,
        rng: &mut Rng,
    ) -> Vec<Particle> {
        let mut particles = Vec::new();
        // 18 particles

        // 0 1 2 3 4
        // 2 4 6 4 2

        // -2 + 0 = -2
        // -2 + 1 = -1
        // -2 + 2 = 0
        // -2 + 3 = 1
        // -2 + 4 = 2

        let v = [2, 4, 6, 4, 2];

        let rot90 = rotation.rotate_90();
        let rad90 = cordic::atan2(rot90.y, rot90.x);

        let zero = FlintVec2::new(Flint::ZERO, Flint::ZERO);

        let neg = -Flint::from_num(v.len() / 2);

        for i in 0..v.len() {
            let idx = Flint::from_num(i);

            for j in 0..v[i] {
                let pos = FlintVec2::new(neg + idx, Flint::ZERO).rotate(&rad90, &zero);
                let c = *centroid + pos;
                let rc = render_centroid - RenderVector2::new(pos.x.to_num(), pos.y.to_num());
                let speed = min_speed + Flint::from_num(rng.i32(0..6));
                let lifetime = rng.i32(2..6) + j;
                let color = RenderColor::LIGHTSKYBLUE;
                let amount = 0;
                let p = self.spawn_particle(
                    &c,
                    rc,
                    rotation,
                    speed,
                    relative_speed,
                    lifetime,
                    color,
                    amount,
                );

                particles.push(p);
            }
        }

        particles
    }

    pub fn spawn_star(
        &self,
        centroid: &FlintVec2,
        rotation: FlintVec2,
        counter: u8,
        amount: u8,
        toggle: bool,
        width: Flint,
        height: Flint,
        color: RenderColor,
    ) -> Star {
        let shape = Shape {
            shape: FlintRectangle::from_centroid(centroid, width, height),
            rotation,
        };

        let body = Body {
            live: shape,
            past: shape,
            dirty: true,
            axes: Vec::new(),
        };

        let render = Renderable::<RenderRectangle>::new(
            color,
            body.live.shape.into(),
            rotation.y.to_num::<f32>().atan2(rotation.x.to_num()),
        );

        Star {
            body,
            render,
            counter,
            amount,
            toggle,
        }
    }

    pub fn spawn_explosion_particles(
        &self,
        centroid: &FlintVec2,
        amount: u8,
        rng: &mut Rng,
    ) -> Vec<Particle> {
        let mut explosion = Vec::new();

        for _ in 0..amount {
            let degrees = rng.i32(0..360);
            let radians = degrees * (Flint::PI / 180);
            let rotation = FlintVec2::new(cordic::cos(radians), cordic::sin(radians));
            let rc = RenderVector2::new(centroid.x.to_num(), centroid.y.to_num());
            let speed = Flint::from_num(rng.i32(50..500)) / 100;
            let relative_speed = Flint::from_num(0);
            // let lifetime = rng.i32(6..18);
            let color = RenderColor::new(
                rng.u8(240..=255),
                rng.u8(180..200),
                rng.u8(0..32),
                rng.u8(240..=255),
            );
            let amount = rng.u8(24..56);
            let lifetime = (color.a / amount) as i32;
            let p = self.spawn_particle(
                centroid,
                rc,
                rotation,
                speed,
                relative_speed,
                lifetime,
                color,
                amount,
            );

            explosion.push(p);
        }

        explosion
    }
}
