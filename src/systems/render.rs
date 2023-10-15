use raylib::prelude::{RaylibDraw, RaylibMode2D};

use crate::{
    components::render::{RenderColor, RenderRectangle, RenderTriangle, RenderVector2},
    engine::Engine,
    entities::{Entities, Particle, Projectile, Triship},
    misc::RaylibRenderHandle,
    world::Map,
};

pub struct RenderSystem;

impl RenderSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        map: &Map,
        entities: &Entities,
        delta: f32,
    ) {
        self.draw_world(rrh, map, delta);

        self.draw_triships(rrh, map, &entities.players, delta);
        self.draw_projectiles(rrh, map, &entities.projectiles, delta);
        self.draw_particles(rrh, map, &entities.particles, delta);

        // TODO: debug
        if true {
            self.draw_triships_debug(rrh, map, &entities.players, delta);
            self.draw_projectiles_debug(rrh, map, &entities.projectiles, delta);
            self.draw_particles_debug(rrh, map, &entities.particles, delta);
        }
    }

    fn draw_world(&self, rrh: &mut RaylibMode2D<RaylibRenderHandle>, map: &Map, _delta: f32) {
        // draw world outlines
        rrh.draw_rectangle_lines(0, 0, map.width_i32, map.height_i32, RenderColor::GREEN);
    }

    fn draw_particles(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        map: &Map,
        particles: &[Particle],
        delta: f32,
    ) {
        for (_, particle) in particles.iter().enumerate() {
            let par = particle.render.lerp(delta);

            if !is_visible_vector2(&par, map) {
                continue;
            }

            rrh.draw_pixel(par.x as i32, par.y as i32, particle.render.color);
        }
    }

    fn draw_particles_debug(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        _map: &Map,
        particles: &[Particle],
        _delta: f32,
    ) {
        return;
        for (_, particle) in particles.iter().enumerate() {
            rrh.draw_pixel(
                particle.body.shape.x.to_num::<i32>(),
                particle.body.shape.y.to_num::<i32>(),
                Engine::DEBUG_TEXT_COLOR,
            );
        }
    }

    fn draw_triships(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        map: &Map,
        triships: &[Triship],
        delta: f32,
    ) {
        for (_, triship) in triships.iter().enumerate() {
            let tri = RenderTriangle {
                v1: triship.render.lerp_v1(delta),
                v2: triship.render.lerp_v2(delta),
                v3: triship.render.lerp_v3(delta),
            };

            if !is_visible_triangle(&tri, map) {
                continue;
            }

            rrh.draw_triangle_lines(tri.v1, tri.v2, tri.v3, triship.render.color);
        }
    }

    fn draw_triships_debug(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        map: &Map,
        triships: &[Triship],
        delta: f32,
    ) {
        for (_, triship) in triships.iter().enumerate() {
            let tri = RenderTriangle {
                v1: triship.render.lerp_v1(delta),
                v2: triship.render.lerp_v2(delta),
                v3: triship.render.lerp_v3(delta),
            };

            if !is_visible_triangle(&tri, map) {
                continue;
            }

            let cen = triship.render.lerp_centroid(delta);
            // TODO: it flickers a little bit sometimes, but maybe fuck it?
            let (x, y) = (cen.x.round() as i32, cen.y.round() as i32);
            let len = triship.body.shape.width.to_num::<i32>()
                + triship.body.shape.height.to_num::<i32>();

            rrh.draw_text(
                &format!(
                    "{} {}",
                    triship.render.live.rotation.to_degrees().round() + 180.0,
                    triship.render.live.rotation
                ),
                x - len,
                y - len,
                10,
                Engine::DEBUG_TEXT_COLOR,
            );

            rrh.draw_text(
                &format!("{}, {}", x, y),
                x + len,
                y,
                10,
                Engine::DEBUG_TEXT_COLOR,
            );

            rrh.draw_text(
                &format!("{} {}", triship.motion.speed, triship.motion.acceleration),
                x - len,
                y + len,
                10,
                Engine::DEBUG_TEXT_COLOR,
            );
        }
    }

    fn draw_projectiles(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        map: &Map,
        projectiles: &[Projectile],
        delta: f32,
    ) {
        for (_, projectile) in projectiles.iter().enumerate() {
            let mut rec = projectile.render.lerp(delta);

            if !is_visible_rectangle(&rec, map) {
                continue;
            }

            rec.x += rec.width / 2.0;
            rec.y += rec.height / 2.0;

            let origin = RenderVector2 {
                x: rec.width / 2.0,
                y: rec.height / 2.0,
            };

            rrh.draw_rectangle_pro(
                rec,
                origin,
                projectile.render.lerp_rotation(delta),
                projectile.render.color,
            );
        }
    }

    fn draw_projectiles_debug(
        &self,
        _rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        map: &Map,
        projectiles: &[Projectile],
        delta: f32,
    ) {
        for (_, projectile) in projectiles.iter().enumerate() {
            let rec = projectile.render.lerp(delta);

            if !is_visible_rectangle(&rec, map) {
                continue;
            }
        }
    }
}

fn is_visible_rectangle(body: &RenderRectangle, map: &Map) -> bool {
    // TODO: rotations

    if body.x + body.width < 0.0 {
        return false;
    }

    if body.x > map.width_f32 {
        return false;
    }

    if body.y + body.height < 0.0 {
        return false;
    }

    if body.y > map.height_f32 {
        return false;
    }

    true
}

fn is_visible_triangle(_body: &RenderTriangle, _map: &Map) -> bool {
    // TODO
    true
}

fn is_visible_vector2(body: &RenderVector2, map: &Map) -> bool {
    if body.x < 0.0 {
        return false;
    }

    if body.x > map.width_f32 {
        return false;
    }

    if body.y < 0.0 {
        return false;
    }

    if body.y > map.height_f32 {
        return false;
    }

    true
}
