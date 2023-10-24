use fastrand::Rng;

use crate::{
    components::render::RenderVector2,
    entities::Entities,
    math::{Flint, FlintVec2},
    spawner::Spawner,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Command {
    Nop,
    RotateLeft,
    RotateRight,
    Accelerate,
    Decelerate,
    Shoot,
    Explode,
}

impl Command {
    pub fn exec(&self, pid: usize, entities: &mut Entities, spawner: &Spawner, rng: &mut Rng) {
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

                // spawn thrust particles

                // get the unrotated "bottom middle"
                // TODO: + one unit below to not make the particles spawn inside the ship
                let centroid = FlintVec2 {
                    x: (p.body.shape.v1.x + p.body.shape.v3.x) / 2,
                    y: (p.body.shape.v1.y + p.body.shape.v3.y) / 2,
                };

                // make sure it's rotated correctly
                let centroid =
                    centroid.rotate(&p.body.rotation.radians(), &p.body.shape.get_centroid());

                let rotation = p.body.rotation.rotate_180();

                // to make the initial rendering look correct we also need to adjust
                // where we put the render centroid
                let mut render_centroid = RenderVector2 {
                    x: (((p.render.past.shape.v1.x + p.render.live.shape.v1.x) / 2.0)
                        + ((p.render.past.shape.v3.x + p.render.live.shape.v3.x) / 2.0))
                        / 2.0,
                    y: (((p.render.past.shape.v1.y + p.render.live.shape.v1.y) / 2.0)
                        + ((p.render.past.shape.v3.y + p.render.live.shape.v3.y) / 2.0))
                        / 2.0,
                };

                // in case ship is accelerating from a negative speed,
                // we need to adjust the relative speed to not make the particles appear inside the ship
                let relative_speed = if p.motion.speed < Flint::ZERO {
                    let s = p.motion.speed * -1 + p.motion.acceleration;
                    let ss = s.to_num::<f32>();
                    render_centroid.x += ss * rotation.x.to_num::<f32>();
                    render_centroid.y += ss * rotation.y.to_num::<f32>();
                    s
                } else {
                    let s = -p.motion.speed;
                    let ss = s.to_num::<f32>();
                    let (sin, cos) = p.render.live.rotation.sin_cos();
                    render_centroid.x += ss * cos;
                    render_centroid.y += ss * sin;
                    s
                };

                let speed = Flint::from_num(0.12);

                let particles = spawner.spawn_thruster_particles(
                    &centroid,
                    render_centroid,
                    rotation,
                    speed,
                    relative_speed,
                    rng,
                );

                entities.particles.extend(particles);
            }
            Command::Decelerate => {
                p.motion.speed -= p.motion.acceleration / 2;

                if p.motion.speed < -p.motion.max_speed / 2 {
                    p.motion.speed = -p.motion.max_speed / 2;
                }
            }
            Command::Shoot => {
                // let's put the projectile a little bit in front of the ship,
                // first we need to get the rotated tip of the ship
                let radians = cordic::atan2(p.body.rotation.y, p.body.rotation.x);
                let mut centroid = p
                    .body
                    .shape
                    .v2
                    .rotate(&radians, &p.body.shape.get_centroid());

                // then we apply the calculated distance to the centroid
                let distance = Flint::from_num(2);

                centroid.x += distance * p.body.rotation.x;
                centroid.y += distance * p.body.rotation.y;

                // to make the initial rendering look correct we also need to adjust
                // where we put the render centroid
                let render_distance = distance.to_num::<f32>();
                let mut render_centroid = p.render.live.shape.v2;
                let (sin, cos) = p.render.live.rotation.sin_cos();
                // TODO: look into why this seems to work, why 0.4? wat
                render_centroid.x += render_distance * (cos - 0.4);
                render_centroid.y += render_distance * (sin - 0.4);

                let projectile = spawner.spawn_projectile(
                    &centroid,
                    p.body.rotation.clone(),
                    render_centroid,
                    p.motion.speed,
                    pid,
                );

                entities.projectiles.push(projectile);
            }
            Command::Explode => {
                let explosion = spawner.spawn_explosion_particles(
                    &FlintVec2::new(Flint::from_num(300), Flint::from_num(300)),
                    16,
                    rng,
                );
                entities.particles.extend(explosion);

                let explosion = spawner.spawn_explosion_particles(
                    &FlintVec2::new(Flint::from_num(500), Flint::from_num(300)),
                    128,
                    rng,
                );
                entities.particles.extend(explosion);
            }
        }
    }
}
