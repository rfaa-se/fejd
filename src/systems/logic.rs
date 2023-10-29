use fastrand::Rng;

use crate::{
    components::{
        logic::{Body, Motion},
        render::RenderColor,
    },
    entities::Entities,
    math::{Flint, FlintRectangle, FlintTriangle, FlintVec2},
    spawner::Spawner,
    world::Map,
};

pub struct LogicSystem {
    deceleration: Flint,
}

impl LogicSystem {
    pub fn new() -> Self {
        LogicSystem {
            deceleration: Flint::from_num(0.06),
        }
    }

    pub fn update(&self, map: &Map, entities: &mut Entities, spawner: &Spawner, rng: &mut Rng) {
        // RENDER
        // update render past bodies,
        // this is so we can interpolate between past and live bodies when drawing
        self.update_render_past(entities);

        // LOGIC
        // update game logic
        self.update_collision_detection(entities, spawner, rng);
        self.update_dead(entities);
        self.update_motion(map, entities);
        self.update_lifetime(entities);
        self.update_out_of_bounds(map, entities);
        self.update_counter_toggle(entities);

        // RENDER
        self.update_color_alpha(entities);
        // update render live bodies,
        // this is so we can interpolate between past and live bodies when drawing
        self.update_render_live(entities);
    }

    fn update_collision_detection(
        &self,
        entities: &mut Entities,
        spawner: &Spawner,
        rng: &mut Rng,
    ) {
        // TODO: fix quad or kd tree for collisions

        // for projectile in entities.projectiles.iter_mut() {
        //     for player in entities.players.iter_mut() {
        //         if projectile.body.intersects(&player.body) {
        //             player.life -= projectile.dmg;
        //         }
        //     }
        // }

        // projectile - player
        for projectile in entities.projectiles.iter_mut() {
            for player in entities.players.iter_mut() {
                // let point = match get_collision_point_rec_tri(
                //     &projectile.body,
                //     &projectile.motion,
                //     &player.body,
                // ) {
                //     Some(point) => point,
                //     None => continue,
                // };

                let shape_alpha = projectile.body.get_axes();
                let shape_beta = player.body.get_axes();

                if !crate::collisions::intersects(shape_alpha, shape_beta) {
                    continue;
                }

                projectile.dead = true;
                player.life -= projectile.dmg;

                // let's go with the projectile point for now
                // TODO: projectile top?
                let point = projectile.body.shape.point;

                let explosion = spawner.spawn_explosion_particles(&point, 32, rng);
                entities.particles.extend(explosion);

                if player.life > 0 {
                    continue;
                }

                player.dead = true;
                let explosion =
                    spawner.spawn_explosion_particles(&player.body.shape.get_centroid(), 128, rng);
                entities.particles.extend(explosion);
            }
        }

        // player - player
        // for i in 0..entities.players.len() - 1 {
        //     let (left, right) = entities.players.split_at_mut(i + 1);
        //     let p1 = &mut left[i];

        //     for p2 in right.iter_mut() {
        //         p2.dead = false;
        //         p1.dead = false;
        //     }
        // }
    }

    fn update_color_alpha(&self, entities: &mut Entities) {
        // particles
        entities
            .particles
            .iter_mut()
            .for_each(|x| apply_color_alpha_twinkle(&mut x.render.color, x.amount, false, true));

        // stars
        entities.stars.iter_mut().for_each(|x| {
            apply_color_alpha_twinkle(&mut x.render.color, x.amount, x.toggle, false)
        });
    }

    fn update_motion(&self, _map: &Map, entities: &mut Entities) {
        // players
        entities.players.iter_mut().for_each(|x| {
            apply_velocity_triangle(&mut x.body, &x.motion);
            apply_deceleration(&mut x.motion, &self.deceleration);
        });

        // projectiles
        entities.projectiles.iter_mut().for_each(|x| {
            apply_velocity_rectangle(&mut x.body, &x.motion);
            // no deceleration on projectiles
        });

        // particles
        entities.particles.iter_mut().for_each(|x| {
            apply_velocity_vector2(&mut x.body, &x.motion);
            // no deceleration on particles
        });

        // stars
        entities.stars.iter_mut().for_each(|x| {
            apply_velocity_rectangle(&mut x.body, &x.motion);
        });
    }

    fn update_render_past(&self, entities: &mut Entities) {
        // players
        entities
            .players
            .iter_mut()
            .for_each(|x| x.render.past = x.render.live);

        // projectiles
        entities
            .projectiles
            .iter_mut()
            .for_each(|x| x.render.past = x.render.live);

        // particles
        entities
            .particles
            .iter_mut()
            .for_each(|x| x.render.past = x.render.live);

        // stars
        entities
            .stars
            .iter_mut()
            .for_each(|x| x.render.past = x.render.live);
    }

    fn update_render_live(&self, entities: &mut Entities) {
        // players
        entities
            .players
            .iter_mut()
            .for_each(|x| x.render.live = (&x.body).into());

        // projectiles
        entities
            .projectiles
            .iter_mut()
            .for_each(|x| x.render.live = (&x.body).into());

        // particles
        entities
            .particles
            .iter_mut()
            .for_each(|x| x.render.live = (&x.body).into());

        // stars
        entities
            .stars
            .iter_mut()
            .for_each(|x| x.render.live = (&x.body).into());
    }

    fn update_out_of_bounds(&self, map: &Map, entities: &mut Entities) {
        entities.projectiles.iter_mut().for_each(|x| {
            if is_out_of_bounds_rectangle(&x.body, map) {
                x.dead = true;
            }
        });
    }

    fn update_dead(&self, entities: &mut Entities) {
        // players
        entities.players.retain(|x| !x.dead);

        // projectiles
        entities.projectiles.retain(|x| !x.dead);

        // particles
        entities.particles.retain(|x| !x.dead);
    }

    fn update_lifetime(&self, entities: &mut Entities) {
        // particles
        entities
            .particles
            .iter_mut()
            .for_each(|x| apply_lifetime_decrease(&mut x.lifetime, &mut x.dead));
    }

    fn update_counter_toggle(&self, entities: &mut Entities) {
        // stars
        entities
            .stars
            .iter_mut()
            .for_each(|x| apply_counter_toggle(&mut x.counter, &mut x.toggle));
    }
}

fn apply_counter_toggle(counter: &mut u8, toggle: &mut bool) {
    if *toggle {
        if *counter != u8::MAX {
            *counter += 1;
        }
    } else {
        if *counter != u8::MIN {
            *counter -= 1;
        }
    }

    if *counter == u8::MAX || *counter == u8::MIN {
        *toggle = !*toggle;
    }
}

fn apply_color_alpha_twinkle(color: &mut RenderColor, amount: u8, add: bool, minmax: bool) {
    if add {
        if color.a <= u8::MAX - amount {
            color.a += amount;
        } else if minmax {
            color.a = u8::MAX;
        }
    } else {
        if color.a >= amount {
            color.a -= amount;
        } else if minmax {
            color.a = u8::MIN;
        }
    }
}

fn apply_lifetime_decrease(lifetime: &mut i32, dead: &mut bool) {
    *lifetime -= 1;

    if *lifetime < 1 {
        *dead = true;
    }
}

fn apply_deceleration(motion: &mut Motion, deceleration: &Flint) {
    // apply deceleration until full stop
    if motion.speed > Flint::ZERO {
        motion.speed -= deceleration;

        if motion.speed < Flint::ZERO {
            motion.speed = Flint::ZERO;
        }
    } else if motion.speed < Flint::ZERO {
        motion.speed += deceleration;

        if motion.speed > Flint::ZERO {
            motion.speed = Flint::ZERO;
        }
    }
}

fn apply_velocity_triangle(body: &mut Body<FlintTriangle>, motion: &Motion) {
    let velocity = body.rotation * motion.speed;

    body.shape.v1 += velocity;
    body.shape.v2 += velocity;
    body.shape.v3 += velocity;
}

fn apply_velocity_rectangle(body: &mut Body<FlintRectangle>, motion: &Motion) {
    let velocity = body.rotation * motion.speed;

    body.shape.point += velocity;
}

fn apply_velocity_vector2(body: &mut Body<FlintVec2>, motion: &Motion) {
    let velocity = body.rotation * motion.speed;

    body.shape += velocity;
}

fn is_out_of_bounds_rectangle(body: &Body<FlintRectangle>, map: &Map) -> bool {
    // TODO: rotations

    if body.shape.point.x + body.shape.width < Flint::ZERO {
        return true;
    }

    if body.shape.point.x > map.width {
        return true;
    }

    if body.shape.point.y + body.shape.height < Flint::ZERO {
        return true;
    }

    if body.shape.point.y > map.height {
        return true;
    }

    false
}

fn get_collision_point_rec_tri(
    _rec: &Body<FlintRectangle>,
    _rec_motion: &Motion,
    _tri: &Body<FlintTriangle>,
) -> Option<FlintVec2> {
    None
}
