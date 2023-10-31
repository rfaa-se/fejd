use fastrand::Rng;

use crate::{
    collisions,
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
        // LOGIC
        // remove dead entities
        self.update_dead(entities);

        // RENDER
        // update render past bodies,
        // this is so we can interpolate between past and live bodies when drawing
        self.update_render_past(entities);

        // LOGIC
        // update game logic
        self.update_body_past(entities);

        self.update_motion(map, entities);
        self.update_lifetime(entities);
        self.update_out_of_bounds(map, entities);
        self.update_counter_toggle(entities);

        self.update_collision_detection(entities, spawner, rng);

        // RENDER
        self.update_color(entities);
        self.update_color(entities);
        // update render live bodies,
        // this is so we can interpolate between past and live bodies when drawing
        self.update_render_live(entities);
    }

    fn update_body_past(&self, entities: &mut Entities) {
        entities
            .players
            .iter_mut()
            .for_each(|x| x.body.past = x.body.live);

        entities
            .projectiles
            .iter_mut()
            .for_each(|x| x.body.past = x.body.live);
    }

    fn update_collision_detection(
        &self,
        entities: &mut Entities,
        spawner: &Spawner,
        rng: &mut Rng,
    ) {
        // TODO: fix quad or kd tree for collisions

        // projectile - player
        for projectile in entities.projectiles.iter_mut() {
            for player in entities.players.iter_mut() {
                let shape_alpha = projectile.body.calc_axes(true);
                let shape_beta = player.body.calc_axes();

                if !collisions::intersects(shape_alpha, shape_beta) {
                    continue;
                }

                projectile.dead = true;
                player.life -= projectile.dmg;

                // if we have a collision we must calculate where we collide,
                // since the projectile includes past and live body to detect
                // a collision we will begin with the past body and move it
                // until we find a collision

                projectile.body.live = projectile.body.past;
                projectile.body.dirty = true;
                let rotation = projectile.body.live.direction;
                let shape_alpha = projectile.body.calc_axes(false);
                let speed = collisions::calculate_speed_to_collision(
                    rotation,
                    projectile.motion.speed,
                    shape_alpha,
                    shape_beta,
                );

                let velocity = rotation * speed;
                projectile.body.live.shape.point += velocity;
                projectile.body.dirty = true;
                // projectile.body.calc_axes(true);

                let explosion =
                    spawner.spawn_explosion_particles(projectile.body.live.shape.point, 32, rng);
                entities.particles.extend(explosion);

                if player.life > 0 {
                    continue;
                }

                player.dead = true;
                // let explosion = spawner.spawn_explosion_particles(
                //     &player.body.live.shape.get_centroid(),
                //     128,
                //     rng,
                // );
                //entities.particles.extend(explosion);
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

    fn update_color(&self, entities: &mut Entities) {
        // particles
        entities.particles.iter_mut().for_each(|x| {
            apply_amount_incdec(&mut x.render.color.a, x.amount, false, true);
            apply_amount_incdec(&mut x.render.color.g, x.amount, true, true);
        });

        // stars
        entities
            .stars
            .iter_mut()
            .for_each(|x| apply_amount_incdec(&mut x.render.color.a, x.amount, x.toggle, false));
    }

    fn update_motion(&self, _map: &Map, entities: &mut Entities) {
        // players
        entities.players.iter_mut().for_each(|x| {
            apply_velocity_triangle(&mut x.body, &x.motion);
            apply_deceleration(&mut x.motion, &self.deceleration);

            x.body.dirty = true; //x.motion.speed != Flint::ZERO;
        });

        // projectiles
        entities.projectiles.iter_mut().for_each(|x| {
            apply_velocity_rectangle(&mut x.body, &x.motion);
            // no deceleration on projectiles

            x.body.dirty = true; //x.motion.speed != Flint::ZERO;
        });

        // particles
        entities.particles.iter_mut().for_each(|x| {
            apply_velocity_vector2(&mut x.body, &x.motion);
            // no deceleration on particles
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
        //entities.players.retain(|x| !x.dead);

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

fn apply_amount_incdec(number: &mut u8, amount: u8, add: bool, minmax: bool) {
    if add {
        if *number <= u8::MAX - amount {
            *number += amount;
        } else if minmax {
            *number = u8::MAX;
        }
    } else {
        if *number >= amount {
            *number -= amount;
        } else if minmax {
            *number = u8::MIN;
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
    let velocity = body.live.direction * motion.speed;

    body.live.shape.v1 += velocity;
    body.live.shape.v2 += velocity;
    body.live.shape.v3 += velocity;
}

fn apply_velocity_rectangle(body: &mut Body<FlintRectangle>, motion: &Motion) {
    let velocity = body.live.direction * motion.speed;

    body.live.shape.point += velocity;
}

fn apply_velocity_vector2(body: &mut Body<FlintVec2>, motion: &Motion) {
    let velocity = body.live.direction * motion.speed;

    body.live.shape += velocity;
}

fn is_out_of_bounds_rectangle(body: &Body<FlintRectangle>, map: &Map) -> bool {
    // TODO: rotations

    if body.live.shape.point.x + body.live.shape.width < Flint::ZERO {
        return true;
    }

    if body.live.shape.point.x > map.width {
        return true;
    }

    if body.live.shape.point.y + body.live.shape.height < Flint::ZERO {
        return true;
    }

    if body.live.shape.point.y > map.height {
        return true;
    }

    false
}
