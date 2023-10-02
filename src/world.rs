use fastrand::Rng;
use raylib::prelude::*;

use crate::{
    commands::Command,
    components::Body,
    engine::Engine,
    entities::{Entities, Player},
    math::{Flint, FlintTriangle, FlintVec2},
    misc::RaylibRenderHandle,
    renderables::Renderable,
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

            let body = Body {
                shape: FlintTriangle::from_centroid(
                    &spawn.point,
                    Flint::from_num(27),
                    Flint::from_num(31),
                ),
                rotation: spawn.rotation,
            };

            let render = Renderable::new(
                Color::GREEN,
                &body.shape.into(),
                spawn
                    .rotation
                    .y
                    .to_num::<f32>()
                    .atan2(spawn.rotation.x.to_num()),
            );

            map.entities.players.push(Player {
                body,
                rotation_speed: Flint::from_num(0.12),
                render,
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

    pub fn update(&mut self, cmds: &[Vec<Command>]) {
        // let _pid = match self.pid {
        //     Some(pid) => pid,
        //     None => return,
        // };

        let mut map = match self.map.as_mut() {
            Some(map) => map,
            None => return,
        };

        // update renderable past bodies,
        // this is so we can interpolate between past and live bodies
        for player in map.entities.players.iter_mut() {
            player.render.past = player.render.live;
        }

        // execute all player commands
        for (pid, cmds) in cmds.iter().enumerate() {
            for cmd in cmds.iter() {
                cmd.exec(pid, &mut map);
            }
        }

        // update world
        // TODO: this is where acceleration, hit collision, etc, is calculated

        // update renderable live bodies
        for player in map.entities.players.iter_mut() {
            player.render.live = player.body.into();
        }
    }

    pub fn draw(&mut self, rrh: &mut RaylibRenderHandle, delta: f32) {
        let (map, pid) = match (&self.map, &self.pid) {
            (Some(map), Some(pid)) => (map, pid),
            _ => return,
        };

        // make camera follow player
        let player = &map.entities.players[*pid];
        let target = player.render.lerp_centroid(delta);

        self.camera.target.x = target.x - (Engine::WIDTH / 2) as f32;
        self.camera.target.y = target.y - (Engine::HEIGHT / 2) as f32;

        {
            let mut rrh = rrh.begin_mode2D(self.camera);

            // TODO: cull entities not currently shown on screen

            // draw world outlines
            rrh.draw_rectangle_lines(0, 0, map.width, map.height, Color::GREEN);

            // draw the players
            for (_, player) in map.entities.players.iter().enumerate() {
                player.render.draw(&mut rrh, delta);
            }
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
