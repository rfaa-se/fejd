use raylib::{
    prelude::{Color, RaylibDraw, RaylibDrawHandle},
    RaylibHandle, RaylibThread,
};

use crate::{
    bus::Bus,
    logs::LogManager,
    messages::{EngineRequestMessage, Message, RequestMessage, Sender, StateRequestMessage},
    states::{State, StateManager},
};

use self::manager::EngineManager;

mod manager;

pub struct Engine {
    managers: Managers,
    bus: Bus,
    ticks: u64,
}

pub struct Managers {
    pub engine: EngineManager,
    pub log: LogManager,
    pub state: StateManager,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            managers: Managers {
                engine: EngineManager::new(),
                log: LogManager::new(),
                state: StateManager::new(),
            },
            bus: Bus::new(),
            ticks: 0,
        }
    }

    pub fn run(&mut self, rh: &mut RaylibHandle, rt: &RaylibThread) {
        // initialize everything before we start
        self.init();

        // TODO: use a render texture instead of drawing directly to screen
        //        let mut render_texture = rh.load_render_texture(rt, 1280, 768).unwrap();

        let mut accumulator = 0.0;

        while !rh.window_should_close() {
            let t = rh.get_frame_time();

            accumulator += t;

            // deal with input as often as possible
            self.input(rh);

            // update engine at a fixed interval
            while accumulator > self.managers.engine.size {
                accumulator -= self.managers.engine.size;
                self.ticks += 1;

                self.update();
            }

            let mut rdh = rh.begin_drawing(&rt);

            // delta is used to smooth interpolation
            let delta = accumulator / self.managers.engine.size;

            // draw as often as possible
            self.draw(&mut rdh, delta);
        }
    }

    fn init(&mut self) {
        // set the ticks per second to 16 by default
        self.bus.send(Message::Request(RequestMessage::Engine(
            EngineRequestMessage::SetTicksPerSecond(16),
        )));

        // enable debug mode by default
        self.bus.send(Message::Request(RequestMessage::Engine(
            EngineRequestMessage::SetDebug(true),
        )));

        // set the state to menu by default
        self.bus.send(Message::Request(RequestMessage::State(
            StateRequestMessage::SetState(State::Menu),
        )));
    }

    fn input(&mut self, rh: &RaylibHandle) {
        self.managers.state.input(rh);
    }

    fn update(&mut self) {
        self.managers.log.update(self.bus.with_sender(Sender::Log));

        self.managers
            .engine
            .update(self.bus.with_sender(Sender::Engine));

        self.managers
            .state
            .update(self.bus.with_sender(Sender::State));

        self.bus.update(&mut self.managers);
    }

    fn draw(&mut self, rdh: &mut RaylibDrawHandle, delta: f32) {
        rdh.clear_background(Color::BLACK);

        self.managers.state.draw(rdh, delta);

        if self.managers.engine.debug {
            // TODO: calc current tps
            let strings = [
                &format!("tps {} {} {}", 16, self.managers.engine.tps, self.ticks),
                &format!("fps {}", rdh.get_fps()),
                &format!("dbg {}", self.managers.engine.debug),
            ];

            let mut y = 4;
            for string in strings {
                rdh.draw_text(string, 4, y, 10, Color::WHITESMOKE);
                y += 10;
            }
        }
    }
}
