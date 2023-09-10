use std::collections::BTreeSet;

use raylib::prelude::*;

use crate::{
    bus::Bus,
    commands::Command,
    entities::Entities,
    math::{Flint, FlintVec2},
    messages::{Message, RequestMessage, Sender, StateRequestMessage},
    misc::RaylibRenderHandle,
    world::{Map, Spawn, World},
};

use super::State;

pub struct GameState {
    actions: BTreeSet<Action>,
    world: World,
    pid: u8,
    players: u8,
    init: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    Initialize { pid: u8, players: u8, seed: u64 },
    GotoMenu,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            world: World::new(),
            actions: BTreeSet::new(),
            pid: 0,
            players: 0,
            init: false,
        }
    }

    pub fn init(&mut self) {
        // TODO: we need to get the pid, pids, and seed
        // this should be fetched from somewhere,
        // when networking is implemented
        self.actions.insert(Action::Initialize {
            pid: fastrand::u8(0..4),
            players: 4,
            seed: fastrand::u64(0..1024),
        });
    }

    pub fn exit(&mut self) {
        self.world.exit();
        self.actions.clear();
        self.pid = 0;
        self.players = 0;
        self.init = false;
    }

    pub fn input(&mut self, rh: &RaylibHandle) {
        if rh.is_key_pressed(KeyboardKey::KEY_E) {
            self.actions.insert(Action::GotoMenu);
        }
    }

    pub fn update(&mut self, bus: &mut Bus) {
        self.action(bus);

        if !self.init {
            return;
        }

        let cmds = vec![vec![Command::Nop]];

        self.world.update(&cmds);
    }

    pub fn message(&mut self, _sender: &Sender, _msg: &Message) {}

    pub fn draw(&mut self, rrh: &mut RaylibRenderHandle, delta: f32) {
        if !self.init {
            return;
        }

        self.world.draw(rrh, delta);
    }

    fn action(&mut self, bus: &mut Bus) {
        while let Some(action) = self.actions.pop_first() {
            match action {
                Action::Initialize { pid, players, seed } => {
                    // TODO: this should be configurable
                    let width = 600;
                    let height = 600;
                    let map = Map {
                        // four spawn points
                        spawns: vec![
                            // top left
                            Spawn {
                                point: FlintVec2::new(Flint::from_num(100), Flint::from_num(100)),
                                rotation: FlintVec2::new(Flint::from_num(0), Flint::from_num(0)),
                            },
                            // top right
                            Spawn {
                                point: FlintVec2::new(
                                    Flint::from_num(width - 100),
                                    Flint::from_num(100),
                                ),
                                rotation: FlintVec2::new(Flint::from_num(0), Flint::from_num(0)),
                            },
                            // bottom left
                            Spawn {
                                point: FlintVec2::new(
                                    Flint::from_num(100),
                                    Flint::from_num(height - 100),
                                ),
                                rotation: FlintVec2::new(Flint::from_num(0), Flint::from_num(0)),
                            },
                            // bottom right
                            Spawn {
                                point: FlintVec2::new(
                                    Flint::from_num(width - 100),
                                    Flint::from_num(height - 100),
                                ),
                                rotation: FlintVec2::new(Flint::from_num(0), Flint::from_num(0)),
                            },
                        ],
                        width,
                        height,
                        entities: Entities::new(),
                    };

                    self.world.init(pid as usize, players as usize, seed, map);

                    self.pid = pid;
                    self.players = players;

                    self.init = true;
                }
                Action::GotoMenu => {
                    bus.send(Message::Request(RequestMessage::State(
                        StateRequestMessage::SetState(State::Menu),
                    )));
                }
            }
        }
    }
}
