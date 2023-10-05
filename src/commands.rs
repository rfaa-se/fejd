use raylib::prelude::Color;

use crate::{
    components::{Body, Motion},
    entities::{Entities, Projectile},
    math::{Flint, FlintRectangle},
    renderables::{RenderRectangle, Renderable},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Command {
    Nop,
    RotateLeft,
    RotateRight,
    Accelerate,
    Decelerate,
    Shoot,
}

impl Command {
    pub fn exec(&self, pid: usize, entities: &mut Entities) {
        let p = match entities.players.get_mut(pid) {
            Some(p) => p,
            None => return,
        };

        match self {
            Command::Nop => (),
            Command::RotateLeft => {
                let rad =
                    cordic::atan2(p.body.rotation.y, p.body.rotation.x) - p.motion.rotation_speed;

                p.body.rotation.x = cordic::cos(rad);
                p.body.rotation.y = cordic::sin(rad);
            }
            Command::RotateRight => {
                let rad =
                    cordic::atan2(p.body.rotation.y, p.body.rotation.x) + p.motion.rotation_speed;

                p.body.rotation.x = cordic::cos(rad);
                p.body.rotation.y = cordic::sin(rad);
            }
            Command::Accelerate => {
                p.motion.speed += p.motion.acceleration;

                if p.motion.speed > p.motion.max_speed {
                    p.motion.speed = p.motion.max_speed;
                }
            }
            Command::Decelerate => {
                p.motion.speed -= p.motion.acceleration / 2;

                if p.motion.speed < -p.motion.max_speed / 2 {
                    p.motion.speed = -p.motion.max_speed / 2;
                }
            }
            Command::Shoot => {
                // TODO: move to something like spawner? entity factory?
                let body = Body {
                    shape: FlintRectangle::from_centroid(
                        &p.body.shape.v2.rotate(
                            &cordic::atan2(p.body.rotation.y, p.body.rotation.x),
                            &p.body.shape.get_centroid(),
                        ),
                        Flint::from_num(2),
                        Flint::from_num(2),
                    ),
                    rotation: p.body.rotation,
                };

                // let's try to make it look like the projectile actually spawns
                // in front of the player
                // TODO: there's still something fucky here
                let mut rec: RenderRectangle = body.shape.into();
                rec.point = p.render.live.shape.v2;

                let render = Renderable::<RenderRectangle>::new(
                    Color::GREEN,
                    &rec,
                    body.rotation
                        .y
                        .to_num::<f32>()
                        .atan2(body.rotation.x.to_num()),
                );

                let speed = Flint::from_num(14);

                let motion = Motion {
                    // projectile will travel at base speed relative to player speed
                    speed: speed + p.motion.speed,
                    max_speed: Flint::MAX,
                    acceleration: speed,
                    rotation_speed: Flint::ZERO,
                };

                entities.projectiles.push(Projectile {
                    body,
                    motion,
                    render,
                    dead: false,
                });
            }
        }
    }
}
