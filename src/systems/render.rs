use raylib::prelude::{Camera2D, RaylibDraw, RaylibMode2D};

use crate::{
    components::render::{RenderColor, RenderRectangle, RenderTriangle, RenderVector2, Renderable},
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
        cam: &Camera2D,
        entities: &Entities,
        debug: bool,
        delta: f32,
    ) {
        self.draw_world(rrh, map, cam, entities, delta);

        entities
            .players
            .iter()
            .filter(|x| !x.dead)
            .for_each(|x| self.draw_triangle(rrh, map, cam, &x.render, delta));

        entities
            .projectiles
            .iter()
            .filter(|x| !x.dead)
            .for_each(|x| self.draw_rectangle(rrh, map, cam, &x.render, delta));

        entities
            .exhausts
            .iter()
            .for_each(|x| self.draw_vector2(rrh, map, cam, &x.render, delta));

        entities
            .explosions
            .iter()
            .for_each(|x| self.draw_vector2(rrh, map, cam, &x.render, delta));

        if !debug {
            return;
        }

        entities
            .players
            .iter()
            .filter(|x| !x.dead)
            .for_each(|x| self.draw_triship_debug(rrh, map, cam, &x, delta));

        entities
            .projectiles
            .iter()
            .filter(|x| !x.dead)
            .for_each(|x| self.draw_projectile_debug(rrh, map, cam, &x, delta));

        entities
            .exhausts
            .iter()
            .for_each(|x| self.draw_particle_debug(rrh, map, cam, &x, delta));

        entities
            .explosions
            .iter()
            .for_each(|x| self.draw_particle_debug(rrh, map, cam, &x, delta));
    }

    fn draw_world(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        map: &Map,
        cam: &Camera2D,
        entities: &Entities,
        _delta: f32,
    ) {
        // draw world outlines
        rrh.draw_rectangle_lines(
            0,
            0,
            map.width_i32,
            map.height_i32,
            Engine::DEBUG_TEXT_COLOR,
        );

        // TODO: fix better stars, make stars loop across the whole world
        let vec = rrh.get_screen_to_world2D(RenderVector2::new(0.0, 0.0), cam);
        let (world_x, world_y) = (vec.x as i32, vec.y as i32);

        // no reason to draw any stars at all if we're outside the world
        if world_x > map.width_i32 || world_y > map.height_i32 {
            return;
        }

        let max_x = if world_x + Engine::WIDTH > map.width_i32 {
            map.width_i32
        } else {
            world_x + Engine::WIDTH
        };

        let max_y = if world_y + Engine::HEIGHT > map.height_i32 {
            map.height_i32
        } else {
            world_y + Engine::HEIGHT
        };

        let star_x = 512;
        let star_y = 512;

        for star in entities.stars.iter() {
            // we will only draw what is currently on the screen,
            // to do that we must find the first valid star position,
            // stars are randomly generated between 0 and 512, in both x and y,
            // draw a repeating star pattern
            // TODO: could probably add some more pseudo randomness here,
            // to not make it look repeated

            let mut x = star.render.live.shape.x as i32;
            let mut y = star.render.live.shape.y as i32;
            let w = star.render.live.shape.width as i32;
            let h = star.render.live.shape.height as i32;

            while x + w < world_x {
                x += star_x;

                if x + w > max_x {
                    x -= star_x;
                    break;
                }
            }

            while y + h < world_y {
                y += star_y;

                if y + h > max_y {
                    y -= star_y;
                    break;
                }
            }

            if x > max_x || y > max_y {
                continue;
            }

            loop {
                loop {
                    rrh.draw_rectangle(x, y, w, h, star.render.color);
                    x += star_x;

                    if x + w > max_x {
                        break;
                    }
                }

                y += star_y;
                x = star.render.live.shape.x as i32;

                if y + h > max_y {
                    break;
                }
            }
        }
    }

    fn draw_vector2(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        map: &Map,
        cam: &Camera2D,
        vec: &Renderable<RenderVector2>,
        delta: f32,
    ) {
        let ren = vec.lerp(delta);

        if !is_visible_vec(&ren, map, cam) {
            return;
        }

        rrh.draw_pixel(ren.x as i32, ren.y as i32, vec.color);
    }

    fn draw_triangle(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        map: &Map,
        cam: &Camera2D,
        tri: &Renderable<RenderTriangle>,
        delta: f32,
    ) {
        let ren = tri.lerp(delta);

        if !is_visible_tri(&ren, map, cam) {
            return;
        }

        // BUG: only triangle_lines work..?
        // rrh.draw_triangle(ren.v1, ren.v2, ren.v3, tri.color);
        // rrh.draw_triangle_fan(&[ren.v1, ren.v2, ren.v3], tri.color);
        // rrh.draw_triangle_strip(&[ren.v1, ren.v2, ren.v3], tri.color);

        rrh.draw_triangle_lines(ren.v1, ren.v2, ren.v3, tri.color);
    }

    fn draw_rectangle(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        map: &Map,
        cam: &Camera2D,
        rec: &Renderable<RenderRectangle>,
        delta: f32,
    ) {
        let mut ren = rec.lerp(delta);

        if !is_visible_rec(&ren, map, cam) {
            return;
        }

        // let w = if ren.width < 2.0 {
        //     ren.width
        // } else {
        //     ren.width / 2.0
        // };

        // let h = if ren.height < 2.0 {
        //     ren.height
        // } else {
        //     ren.height / 2.0
        // };

        // ren.x += w;
        // ren.y += h;

        let origin = RenderVector2 {
            x: ren.width / 2.0,
            y: ren.height / 2.0,
        };

        ren.x += origin.x;
        ren.y += origin.y;

        rrh.draw_rectangle_pro(ren, origin, rec.lerp_angle(delta).to_degrees(), rec.color);
    }

    fn draw_projectile_debug(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        _map: &Map,
        _cam: &Camera2D,
        projectile: &Projectile,
        _delta: f32,
    ) {
        let mut ren = projectile.render.live.shape;

        // let w = if ren.width < 2.0 {
        //     ren.width
        // } else {
        //     ren.width / 2.0
        // };

        // let h = if ren.height < 2.0 {
        //     ren.height
        // } else {
        //     ren.height / 2.0
        // };

        // ren.x += w;
        // ren.y += h;
        let origin = RenderVector2 {
            x: ren.width / 2.0,
            y: ren.height / 2.0,
        };

        ren.x += origin.x;
        ren.y += origin.y;

        let axes = &projectile.body.axes;
        for i in 0..axes.len() {
            let a = axes[i];
            let b = axes[if i + 1 == axes.len() { 0 } else { i + 1 }];
            let c = match i {
                0 => RenderColor::BLUE,
                1 => RenderColor::GREEN,
                2 => RenderColor::RED,
                _ => RenderColor::ORANGE,
            };

            let v1 = Into::<RenderVector2>::into(a);
            let v2 = Into::<RenderVector2>::into(b);

            rrh.draw_line_v(v1, v2, c);
        }

        // println!("{:?} {:?} {:?} {:?}", axes[0], axes[1], axes[2], axes[3]);
        // println!("{:?}", ren);

        // let mut c = Engine::DEBUG_TEXT_COLOR;
        // c.a = 80;

        // rrh.draw_rectangle_pro(
        //     ren,
        //     origin,
        //     projectile.render.live.angle.to_degrees(),
        //     c, // Engine::DEBUG_TEXT_COLOR,
        // );
    }

    fn draw_triship_debug(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        map: &Map,
        cam: &Camera2D,
        triship: &Triship,
        delta: f32,
    ) {
        if !is_visible_tri(&triship.render.live.shape, map, cam) {
            return;
        }

        rrh.draw_triangle_lines(
            triship.render.live.shape.v1,
            triship.render.live.shape.v2,
            triship.render.live.shape.v3,
            Engine::DEBUG_TEXT_COLOR,
        );

        let cen = triship.render.lerp_centroid(delta);

        let (x, y) = (cen.x.round() as i32, cen.y.round() as i32);

        let len = triship.body.live.shape.width.to_num::<i32>()
            + triship.body.live.shape.height.to_num::<i32>();

        rrh.draw_text(
            &format!(
                "{} {}",
                triship.render.live.angle.round() + 180.0,
                triship.render.live.angle
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

    fn draw_particle_debug(
        &self,
        rrh: &mut RaylibMode2D<RaylibRenderHandle>,
        _map: &Map,
        _cam: &Camera2D,
        par: &Particle,
        _delta: f32,
    ) {
        rrh.draw_pixel(
            par.render.live.shape.x as i32,
            par.render.live.shape.y as i32,
            Engine::DEBUG_TEXT_COLOR,
        );
    }
}

fn is_visible_rec(body: &RenderRectangle, map: &Map, _cam: &Camera2D) -> bool {
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

fn is_visible_tri(_body: &RenderTriangle, _map: &Map, _cam: &Camera2D) -> bool {
    // TODO
    true
}

fn is_visible_vec(body: &RenderVector2, map: &Map, _cam: &Camera2D) -> bool {
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
