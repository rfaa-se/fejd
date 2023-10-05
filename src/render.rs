use raylib::prelude::{Color, RaylibDraw, RaylibMode2D, Rectangle, Vector2};

use crate::{entities::Entities, misc::RaylibRenderHandle, world::Map};

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

        // TODO: cull entities not currently shown on screen
        self.draw_players(rrh, map, entities, delta);
        self.draw_projectiles(rrh, map, entities, delta);
    }

    fn draw_world(&self, rrh: &mut RaylibMode2D<RaylibRenderHandle>, map: &Map, _delta: f32) {
        // draw world outlines
        rrh.draw_rectangle_lines(0, 0, map.width_i32, map.height_i32, Color::GREEN);
    }

    fn draw_players(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        _map: &Map,
        entities: &Entities,
        delta: f32,
    ) {
        // draw the players
        for (_, player) in entities.players.iter().enumerate() {
            rrh.draw_triangle_lines(
                player.render.lerp_v1(delta),
                player.render.lerp_v2(delta),
                player.render.lerp_v3(delta),
                player.render.color,
            );

            let cen = player.render.lerp_centroid(delta);
            let (x, y) = (cen.x.round() as i32, cen.y.round() as i32);

            rrh.draw_text(
                &format!(
                    "{} {}",
                    player.render.live.rotation.to_degrees().round() + 180.0,
                    player.render.live.rotation
                ),
                x - 24,
                y - 32,
                10,
                Color::WHITESMOKE,
            );

            rrh.draw_text(&format!("{}, {}", x, y), x + 24, y, 10, Color::WHITESMOKE);

            let cen = player.render.lerp_centroid(delta);
            let (x, y) = (cen.x.round() as i32, cen.y.round() as i32);

            // draw some additional debug data
            rrh.draw_text(
                &format!("{} {}", player.motion.speed, player.motion.acceleration),
                x - 14,
                y + 22,
                10,
                Color::WHITESMOKE,
            );
        }
    }

    fn draw_projectiles(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        map: &Map,
        entities: &Entities,
        delta: f32,
    ) {
        for (_, projectile) in entities.projectiles.iter().enumerate() {
            let mut r = projectile.render.lerp(delta);

            if !is_visible_rectangle(&r, map) {
                continue;
            }

            r.x += r.width / 2.0;
            r.y += r.height / 2.0;

            let o = Vector2 {
                x: r.width / 2.0,
                y: r.height / 2.0,
            };

            rrh.draw_rectangle_pro(
                r,
                o,
                projectile.render.lerp_rotation(delta),
                projectile.render.color,
            );
        }
    }
}

fn is_visible_rectangle(body: &Rectangle, map: &Map) -> bool {
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
