use std::collections::{BTreeSet, HashMap};

use raylib::prelude::*;

use crate::{
    bus::Bus,
    commands::Command,
    engine::Engine,
    math::{Directions, Flint, FlintVec2},
    messages::{
        EngineMessage, EngineRequestMessage, Message, RequestMessage, Sender, StateRequestMessage,
    },
    misc::RaylibRenderHandle,
    world::{Map, Spawn, World},
};

use super::State;

pub struct GameState {
    actions: BTreeSet<Action>,
    world: World,
    tick: u64,
    pid: u8,
    players: u8,
    init: bool,
    stalling: bool,
    cmds: Vec<Command>,
    rcmds: HashMap<u64, ReceivedCommands>,
    empty: Vec<Vec<Command>>,
    debug: bool,
    paused: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    Initialize { pid: u8, players: u8, seed: u64 },
    GotoMenu,
    Command(Command),
    GetDebug,
    ToggleDebug,
    TogglePause,
}

struct ReceivedCommands {
    ready: bool,
    received: u8,
    commands: Vec<Vec<Command>>,
}

impl GameState {
    const DELAY_TICKS: u64 = 3;

    pub fn simulate_recv_cmds(&mut self) {
        // TODO: this is temporary until we can get the networking implemented,
        // this will simulate a delay of 3 ticks, which at 16 tps will be
        // (1000 ms / 16) * 3 = 187.5 ms

        let rcmds = self
            .rcmds
            .entry(self.tick + GameState::DELAY_TICKS)
            .or_insert_with(|| ReceivedCommands {
                ready: false,
                received: 0,
                commands: vec![Vec::new(); self.players as usize],
            });

        let mut rotated_left = false;
        let mut rotated_right = false;

        for i in 0..self.players {
            let cmds = &mut rcmds.commands[i as usize];

            if i == self.pid {
                for cmd in self.cmds.drain(..) {
                    cmds.push(cmd);
                }
            } else if !rotated_left {
                rotated_left = true;
                cmds.push(Command::RotateLeft);
            } else if !rotated_right {
                rotated_right = true;
                cmds.push(Command::RotateRight);
            } else {
                cmds.push(Command::Nop);
            }

            rcmds.received += 1;
            rcmds.ready = rcmds.received == self.players;
        }
    }

    pub fn new() -> Self {
        GameState {
            actions: BTreeSet::new(),
            world: World::new(),
            tick: 0,
            pid: 0,
            players: 0,
            init: false,
            stalling: false,
            cmds: Vec::new(),
            rcmds: HashMap::new(),
            empty: Vec::new(),
            debug: false,
            paused: false,
        }
    }

    pub fn init(&mut self) {
        self.actions.insert(Action::GetDebug);

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
        self.cmds.clear();
        self.rcmds.clear();
        self.pid = 0;
        self.players = 0;
        self.tick = 0;
        self.init = false;
        self.stalling = false;
        self.paused = false;
    }

    pub fn input(&mut self, rh: &RaylibHandle) {
        if rh.is_key_pressed(KeyboardKey::KEY_E) {
            self.actions.insert(Action::GotoMenu);
        }

        if rh.is_key_pressed(KeyboardKey::KEY_D) {
            self.actions.insert(Action::ToggleDebug);
        }
        if rh.is_key_pressed(KeyboardKey::KEY_P) {
            self.actions.insert(Action::TogglePause);
        }

        // TODO: make sure we don't insert duplicate commands
        if self.paused {
            return;
        }

        if rh.is_key_down(KeyboardKey::KEY_LEFT) {
            self.actions.insert(Action::Command(Command::RotateLeft));
        }

        if rh.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.actions.insert(Action::Command(Command::RotateRight));
        }

        if rh.is_key_down(KeyboardKey::KEY_UP) {
            self.actions.insert(Action::Command(Command::Accelerate));
        }

        if rh.is_key_down(KeyboardKey::KEY_DOWN) {
            self.actions.insert(Action::Command(Command::Decelerate));
        }

        if rh.is_key_down(KeyboardKey::KEY_SPACE) {
            self.actions.insert(Action::Command(Command::Shoot));
        }

        if rh.is_key_pressed(KeyboardKey::KEY_LEFT_CONTROL) {
            self.actions.insert(Action::Command(Command::Explode));
        }

        if rh.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL) {
            self.actions.insert(Action::Command(Command::Explode));
        }
    }

    pub fn update(&mut self, bus: &mut Bus) {
        self.action(bus);

        if !self.init || self.paused {
            return;
        }

        self.simulate_recv_cmds();

        let cmds = match self.rcmds.get_mut(&self.tick) {
            // once we have received all commands and ready is set,
            // we can proceed with the world
            Some(cmds) if cmds.ready => &mut cmds.commands,
            // we will not have received any data for the first couple of ticks,
            // this is intentional
            None if self.tick < GameState::DELAY_TICKS => &mut self.empty,
            // if we have not yet received all commands for this tick,
            // then we are stalling
            _ => {
                self.stalling = true;
                return;
            }
        };

        self.world.update(&cmds);
        self.tick += 1;

        self.cmds.clear();
    }

    pub fn message(&mut self, _sender: &Sender, msg: &Message) {
        // TODO: once we get all commands for the current tick,
        // set stalling to false

        match msg {
            Message::Engine(EngineMessage::DebugGet(debug) | EngineMessage::DebugSet(debug)) => {
                self.debug = *debug;
            }
            _ => return,
        }
    }

    pub fn draw(&mut self, rrh: &mut RaylibRenderHandle, delta: f32) {
        if !self.init {
            return;
        }

        let delta = if self.paused { 1.0 } else { delta };

        self.world.draw(rrh, self.debug, delta);

        // if self.debug {
        if true {
            let text = format!("{} pid", self.pid);
            rrh.draw_text(
                &text,
                Engine::WIDTH - raylib::text::measure_text(&text, 10) - 4,
                4,
                10,
                Color::WHITESMOKE,
            );

            let text = format!("{} ticks", self.tick);
            rrh.draw_text(
                &text,
                Engine::WIDTH - raylib::text::measure_text(&text, 10) - 4,
                14,
                10,
                Color::WHITESMOKE,
            );
        }
    }

    fn action(&mut self, bus: &mut Bus) {
        while let Some(action) = self.actions.pop_first() {
            match action {
                Action::Initialize { pid, players, seed } => {
                    // TODO: map should be configurable
                    let width = Flint::from_num(400);
                    let height = Flint::from_num(400);
                    let map = Map {
                        // four spawn points for this map
                        spawns: vec![
                            // top left
                            Spawn {
                                point: FlintVec2::new(Flint::from_num(100), Flint::from_num(100)),
                                direction: Directions::WEST,
                            },
                            // top right
                            Spawn {
                                point: FlintVec2::new(
                                    width - Flint::from_num(100),
                                    Flint::from_num(100),
                                ),
                                direction: Directions::SOUTH,
                            },
                            // bottom left
                            Spawn {
                                point: FlintVec2::new(
                                    Flint::from_num(100),
                                    height - Flint::from_num(100),
                                ),
                                direction: Directions::EAST,
                            },
                            // bottom right
                            Spawn {
                                point: FlintVec2::new(
                                    width - Flint::from_num(100),
                                    height - Flint::from_num(100),
                                ),
                                direction: Directions::NORTH,
                            },
                        ],
                        width,
                        height,
                        width_i32: width.to_num(),
                        height_i32: height.to_num(),
                        width_f32: width.to_num(),
                        height_f32: height.to_num(),
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
                Action::Command(cmd) => {
                    // TODO: these should be sent via net
                    self.cmds.push(cmd);
                }
                Action::GetDebug => {
                    bus.send(Message::Request(RequestMessage::Engine(
                        EngineRequestMessage::GetDebug,
                    )));
                }
                Action::ToggleDebug => {
                    bus.send(Message::Request(RequestMessage::Engine(
                        EngineRequestMessage::SetDebug(!self.debug),
                    )));
                }
                Action::TogglePause => {
                    self.paused = !self.paused;
                }
            }
        }
    }
}
