use fastrand::Rng;
use raylib::prelude::*;

use crate::{
    commands::Command,
    components::Body,
    engine::Engine,
    entities::{Entities, Player},
    math::{Flint, FlintTriangle, FlintVec2},
    misc::RaylibRenderHandle,
};

pub struct Spawn {
    pub point: FlintVec2,
    pub rotation: FlintVec2,
}

pub struct Map {
    pub spawns: Vec<Spawn>,
    pub width: i32,
    pub height: i32,
    pub entities: Entities,
}

pub struct World {
    rng: Rng,
    seed: Option<u64>,
    pid: Option<usize>,
    map: Option<Map>,
    camera: Camera2D,
}

impl World {
    pub fn new() -> Self {
        World {
            rng: Rng::new(),
            pid: None,
            seed: None,
            map: None,
            camera: Camera2D {
                offset: Vector2::new(0.0, 0.0),
                target: Vector2::new(0.0, 0.0),
                rotation: 0.0,
                zoom: 1.0,
            },
        }
    }

    pub fn init(&mut self, pid: usize, players: usize, seed: u64, mut map: Map) {
        // players must not be greater than the spawn points in the map
        // TODO: might be fixable without manual checks with const generics somehow, skip for now
        self.rng.seed(seed);

        // randomize spawn points
        let mut positions: Vec<usize> = (0..players).collect();
        self.rng.shuffle(&mut positions);

        for pid in positions.iter().take(players) {
            let spawn = &map.spawns[*pid];

            map.entities.players.push(Player {
                color: Color::GREEN,
                body: Body::new(
                    FlintTriangle::from_centroid(
                        spawn.point,
                        Flint::from_num(27),
                        Flint::from_num(31),
                        spawn.rotation,
                    ),
                    spawn.rotation,
                ),
                // rotation speed is in radians
                rotation_speed: (Flint::from_num(10) / 180) * Flint::PI,
            });
        }

        println!(
            "{} {}",
            map.entities.players[0].body.current.v2.x,
            map.entities.players[0].body.current.get_centroid().x
        );

        println!("ROUND {}", (Flint::from_num(27) / 2).round());
        println!(
            "CENTROID {:?}",
            map.entities.players[0].body.current.get_centroid()
        );

        self.pid = Some(pid);
        self.seed = Some(seed);
        self.map = Some(map);
    }

    pub fn exit(&mut self) {
        self.pid = None;
        self.seed = None;
        self.map = None;
    }

    pub fn update(&mut self, cmds: &[Vec<Command>]) {
        // let _pid = match self.pid {
        //     Some(pid) => pid,
        //     None => return,
        // };

        let mut map = match self.map.as_mut() {
            Some(map) => map,
            None => return,
        };

        for (pid, cmds) in cmds.iter().enumerate() {
            for cmd in cmds.iter() {
                cmd.exec(pid, &mut map);
            }
        }
    }

    pub fn draw(&mut self, rrh: &mut RaylibRenderHandle, delta: f32) {
        let (map, pid) = match (&self.map, &self.pid) {
            (Some(map), Some(pid)) => (map, pid),
            _ => return,
        };

        // make camera follow player
        let player = &map.entities.players[*pid];
        let pos = player.body.lerp_center(delta);

        self.camera.target.x = pos.x - (Engine::WIDTH / 2) as f32;
        self.camera.target.y = pos.y - (Engine::HEIGHT / 2) as f32;

        {
            // draw world
            let mut rdh = rrh.begin_mode2D(self.camera);

            rdh.draw_rectangle_lines(0, 0, map.width, map.height, Color::GREEN);

            for (i, player) in map.entities.players.iter().enumerate() {
                rdh.draw_triangle_lines(
                    player.body.lerp_v1(delta),
                    player.body.lerp_v2(delta),
                    player.body.lerp_v3(delta),
                    player.color,
                );

                rdh.draw_line_v(
                    player.body.lerp_v1(delta),
                    player.body.lerp_v2(delta),
                    Color::BLUE,
                );

                rdh.draw_line_v(
                    player.body.lerp_v2(delta),
                    player.body.lerp_v3(delta),
                    Color::RED,
                );

                rdh.draw_line_v(
                    player.body.lerp_v3(delta),
                    player.body.lerp_v1(delta),
                    Color::YELLOW,
                );

                let point = map.spawns[i].point;
                rdh.draw_pixel(point.x.to_num(), point.y.to_num(), Color::YELLOW);
            }

            // debug stuff
            rdh.draw_pixel(pos.x as i32, pos.y as i32, Color::RED);

            let rad = cordic::atan2(player.body.rotation.y, player.body.rotation.x);
            let rot = (rad * 180) / Flint::PI;
            rdh.draw_rectangle_pro(
                Rectangle {
                    x: pos.x + 30.0,
                    y: pos.y,
                    width: 10.0,
                    height: 10.0,
                },
                Vector2::new(5.0, 5.0),
                rot.to_num(),
                Color::ORANGE,
            );

            rdh.draw_text(
                &format!("{:?}", player.body.rotation),
                pos.x as i32 + 30,
                pos.y as i32 - 20,
                10,
                Color::WHITESMOKE,
            );
        }

        // TODO: debug
        if true {
            let text = format!("pid {}", pid);
            rrh.draw_text(
                &format!("pid {}", pid),
                Engine::WIDTH - raylib::text::measure_text(&text, 10) - 4,
                4,
                10,
                Color::WHITE,
            );
        }
    }
}
