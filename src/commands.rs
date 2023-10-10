use crate::{entities::Entities, math::Flint, misc, spawner::Spawner};

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
    pub fn exec(&self, pid: usize, entities: &mut Entities, spawner: &Spawner) {
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
                // let's put the projectile a little bit in front of the ship,
                // first we need to get the rotated tip of the ship
                let radians = cordic::atan2(p.body.rotation.y, p.body.rotation.x);
                let mut centroid = p
                    .body
                    .shape
                    .v2
                    .rotate(&radians, &p.body.shape.get_centroid());

                // then we apply the calculated distance to the centroid
                let distance = Flint::from_num(6);

                centroid.x += distance * p.body.rotation.x;
                centroid.y += distance * p.body.rotation.y;

                // to make the initial rendering look correct we also need to adjust
                // where we put the render centroid, for this we also need
                // to rotate it
                let render_distance = distance.to_num::<f32>();
                // let mut render_centroid = p.render.live.shape.v2;
                // render_centroid.x += render_distance * p.render.live.rotation.cos();
                // render_centroid.y += render_distance * p.render.live.rotation.sin();
                // let mut render_centroid = misc::rotate_vector2(
                //     &p.render.live.shape.v2,
                //     &p.render.live.rotation,
                //     &p.render.live.shape.get_centroid(),
                // );
                let mut render_centroid = p.render.live.shape.v2;
                let (sin, cos) = p.render.live.rotation.sin_cos();
                render_centroid.x += render_distance * cos;
                render_centroid.y += render_distance * sin;

                // render_centroid.x += render_distance * p.render.live.rotation.cos();
                // render_centroid.y += render_distance * p.render.live.rotation.sin();

                let projectile = spawner.spawn_projectile(
                    &centroid,
                    &p.body.rotation,
                    &render_centroid,
                    p.render.live.rotation,
                    &p.motion.speed,
                );

                entities.projectiles.push(projectile);
            }
        }
    }
}
