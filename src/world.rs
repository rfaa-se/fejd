use fastrand::Rng;
use raylib::prelude::*;

use crate::{
    commands::Command,
    engine::Engine,
    entities::Entities,
    math::{Flint, FlintVec2},
    misc::RaylibRenderHandle,
    spawner::Spawner,
    systems::{LogicSystem, RenderSystem},
};

pub struct Spawn {
    pub point: FlintVec2,
    pub rotation: FlintVec2,
}

pub struct Map {
    pub spawns: Vec<Spawn>,
    pub width: Flint,
    pub height: Flint,
    pub width_i32: i32,
    pub height_i32: i32,
    pub width_f32: f32,
    pub height_f32: f32,
}

pub struct World {
    rng: Rng,
    seed: Option<u64>,
    pid: Option<usize>,
    map: Option<Map>,
    camera: Camera2D,
    tick: u64,
    logic: LogicSystem,
    render: RenderSystem,
    entities: Entities,
    spawner: Spawner,
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
            tick: 0,
            logic: LogicSystem::new(),
            render: RenderSystem::new(),
            entities: Entities::new(),
            spawner: Spawner::new(),
        }
    }

    pub fn init(&mut self, pid: usize, players: usize, seed: u64, map: Map) {
        // players must not be greater than the spawn points in the map
        // TODO: might be fixable without manual checks with const generics somehow, skip for now
        self.rng.seed(seed);

        // randomize spawn points
        let mut positions: Vec<usize> = (0..players).collect();
        self.rng.shuffle(&mut positions);

        for pid in positions.iter().take(players) {
            let spawn = &map.spawns[*pid];
            let player = self.spawner.spawn_triship(&spawn.point, &spawn.rotation);
            self.entities.players.push(player);
        }

        self.pid = Some(pid);
        self.seed = Some(seed);
        self.map = Some(map);
    }

    pub fn exit(&mut self) {
        self.pid = None;
        self.seed = None;
        self.map = None;
        self.tick = 0;
        self.entities.clear();
    }

    pub fn update(&mut self, cmds: &[Vec<Command>]) {
        // let _pid = match self.pid {
        //     Some(pid) => pid,
        //     None => return,
        // };

        let map = match self.map.as_mut() {
            Some(map) => map,
            None => return,
        };

        // execute all player commands
        for (pid, cmds) in cmds.iter().enumerate() {
            for cmd in cmds.iter() {
                cmd.exec(pid, &mut self.entities, &self.spawner);
            }
        }

        // update all logic systems
        self.logic.update(map, &mut self.entities);

        self.tick += 1;
    }

    pub fn draw(&mut self, rrh: &mut RaylibRenderHandle, delta: f32) {
        let (map, pid) = match (&self.map, &self.pid) {
            (Some(map), Some(pid)) => (map, pid),
            _ => return,
        };

        // make camera follow player
        let player = &self.entities.players[*pid];
        let target = player.render.lerp_centroid(delta);

        self.camera.target.x = target.x - Engine::WIDTH as f32 / 2.0;
        self.camera.target.y = target.y - Engine::HEIGHT as f32 / 2.0;

        // draw all render systems
        self.render.draw(
            &mut rrh.begin_mode2D(self.camera),
            map,
            &self.entities,
            delta,
        );

        // draw some debug data
        let text = format!("{} ents", self.entities.get_count());
        rrh.draw_text(
            &text,
            Engine::WIDTH - raylib::text::measure_text(&text, 10) - 4,
            24,
            10,
            Color::WHITESMOKE,
        );
    }
}
