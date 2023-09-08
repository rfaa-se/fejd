use fastrand::Rng;
use raylib::prelude::*;

use crate::{
    commands::Command,
    components::Body,
    entities::{Entities, Player},
    math::{Flint, FlintTriangle, FlintVec2},
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
                position: Body::new(FlintTriangle::from_center(
                    spawn.point,
                    Flint::from_num(27),
                    Flint::from_num(31),
                    spawn.rotation,
                )),
            });
        }

        self.pid = Some(pid);
        self.seed = Some(seed);
        self.map = Some(map);
    }

    pub fn exit(&mut self) {
        self.pid = None;
        self.seed = None;
        self.map = None;
    }

    pub fn update(&mut self, _cmds: &[Vec<Command>]) {
        let _pid = match self.pid {
            Some(pid) => pid,
            None => return,
        };
    }

    pub fn draw(&mut self, rdh: &mut RaylibDrawHandle, delta: f32) {
        let (map, pid) = match (&self.map, &self.pid) {
            (Some(map), Some(pid)) => (map, pid),
            _ => return,
        };

        // make camera follow player
        let player = &map.entities.players[*pid];
        let pos = player.position.lerp_center(delta);

        self.camera.target.x = pos.x - (rdh.get_screen_width() / 2) as f32;
        self.camera.target.y = pos.y - (rdh.get_screen_height() / 2) as f32;

        {
            // draw world
            let mut rdh = rdh.begin_mode2D(self.camera);

            rdh.draw_rectangle_lines(0, 0, map.width, map.height, Color::GREEN);

            for (i, player) in map.entities.players.iter().enumerate() {
                rdh.draw_triangle_lines(
                    player.position.lerp_v1(delta),
                    player.position.lerp_v2(delta),
                    player.position.lerp_v3(delta),
                    player.color,
                );

                let point = map.spawns[i].point;
                rdh.draw_pixel(point.x.to_num(), point.y.to_num(), Color::YELLOW);
            }

            rdh.draw_pixel(pos.x as i32, pos.y as i32, Color::RED);
        }

        // TODO: debug
        if true {
            let text = format!("pid {}", pid);
            rdh.draw_text(
                &format!("pid {}", pid),
                rdh.get_screen_width() - raylib::text::measure_text(&text, 10) - 4,
                4,
                10,
                Color::WHITE,
            );
        }
    }
}
