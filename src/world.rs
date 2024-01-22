use fastrand::Rng;
use raylib::prelude::*;

use crate::{
    bus::Bus,
    commands::Command,
    components::{logic::Miscellaneous, render::RenderColor},
    engine::Engine,
    entities::Entities,
    math::{Directions, Flint, FlintVec2},
    messages::{Message, Sender},
    misc::RaylibRenderHandle,
    spawner::Spawner,
    systems::{LogicSystem, RenderSystem},
};

pub struct Spawn {
    pub point: FlintVec2,
    pub direction: FlintVec2,
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
    misc: Miscellaneous,
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
            misc: Miscellaneous::new(),
        }
    }

    pub fn init(&mut self, pid: usize, players: usize, seed: u64, map: Map) {
        // players must not be greater than the spawn points in the map
        // TODO: might be fixable without manual checks with const generics somehow, skip for now

        // seed the rng so it's synced across clients
        self.rng.seed(seed);

        // randomize spawn points
        let mut positions: Vec<usize> = (0..players).collect();
        self.rng.shuffle(&mut positions);

        // spawn players
        for pid in positions.iter().take(players) {
            let spawn = &map.spawns[*pid];
            let player = self.spawner.spawn_triship(spawn.point, spawn.direction);
            self.entities.players.push(player);
            self.misc.player_map_spawn_indexes.push(*pid);
        }

        // spawn stars
        for _ in 0..64 {
            let width = self.rng.u8(1..2);
            let height = self.rng.u8(1..2);
            let centroid = FlintVec2::new(
                Flint::from_num(self.rng.i32((1 + width as i32)..(512 - width as i32))),
                Flint::from_num(self.rng.i32((1 + height as i32)..(512 - height as i32))),
            );
            let rotation = Directions::NORTH;
            let color = RenderColor::new(
                self.rng.u8(180..255),
                self.rng.u8(180..255),
                self.rng.u8(180..255),
                self.rng.u8(0..255),
            );

            let star = self.spawner.spawn_star(
                centroid,
                rotation,
                self.rng.u8(u8::MIN..u8::MAX),
                self.rng.u8(0..15),
                self.rng.bool(),
                Flint::from_num(width),
                Flint::from_num(height),
                color,
            );

            self.entities.stars.push(star);
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
        self.misc.clear();
    }

    pub fn update(&mut self, cmds: &[Vec<Command>], bus: &mut Bus) {
        // let _pid = match self.pid {
        //     Some(pid) => pid,
        //     None => return,
        // };

        let map = match self.map.as_mut() {
            Some(map) => map,
            None => return,
        };

        // update all logic systems
        self.logic.update(
            map,
            &self.spawner,
            &mut self.entities,
            &mut self.rng,
            &mut self.misc,
            cmds,
            bus.with_sender(Sender::Logic),
        );

        self.tick += 1;
    }

    pub fn message(&mut self, sender: &Sender, msg: &Message) {
        self.logic.message(sender, msg);
    }

    pub fn draw(&mut self, rrh: &mut RaylibRenderHandle, debug: bool, delta: f32) {
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
            &self.camera,
            &self.entities,
            debug,
            delta,
        );

        // if debug {
        if true {
            // draw some debug data
            let text = format!("{} ents", self.entities.count());
            rrh.draw_text(
                &text,
                Engine::WIDTH - raylib::text::measure_text(&text, 10) - 4,
                24,
                10,
                Color::WHITESMOKE,
            );
        }
    }
}
