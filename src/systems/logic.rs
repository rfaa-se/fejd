use crate::{
    components::logic::{Body, Motion},
    entities::Entities,
    math::{Flint, FlintRectangle, FlintTriangle},
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

    pub fn update(&self, map: &Map, entities: &mut Entities) {
        // update render past bodies, this is so we can interpolate between past and live bodies
        self.update_render_past(map, entities);

        // update game logic
        self.update_dead(entities);
        self.update_motion(map, entities);
        self.update_out_of_bounds(map, entities);

        // update render live bodies, this is so we can interpolate between past and live bodies
        self.update_render_live(map, entities);
    }

    fn update_motion(&self, _map: &Map, entities: &mut Entities) {
        // players
        for entity in entities.players.iter_mut() {
            // apply deceleration
            apply_deceleration(&mut entity.motion, &self.deceleration);

            // apply velocity
            apply_velocity_triangle(&mut entity.body, &entity.motion);
        }

        // projectiles
        for entity in entities.projectiles.iter_mut() {
            // no deceleration on projectiles

            // apply velocity
            apply_velocity_rectangle(&mut entity.body, &entity.motion);
        }
    }

    fn update_render_past(&self, _map: &Map, entities: &mut Entities) {
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
    }

    fn update_render_live(&self, _map: &Map, entities: &mut Entities) {
        // players
        entities
            .players
            .iter_mut()
            .for_each(|x| x.render.live = x.body.into());

        // projectiles
        entities
            .projectiles
            .iter_mut()
            .for_each(|x| x.render.live = x.body.into());
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

    // TODO: check if out of bounds
}

fn apply_velocity_rectangle(body: &mut Body<FlintRectangle>, motion: &Motion) {
    // TODO: rotations

    let velocity = body.rotation * motion.speed;

    body.shape.point = body.shape.point + velocity;
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
