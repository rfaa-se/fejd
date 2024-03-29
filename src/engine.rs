use raylib::prelude::*;

use crate::{
    audio::AudioManager,
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
    pub audio: AudioManager,
}

impl Engine {
    pub const DEBUG_TEXT_COLOR: Color = Color::WHITESMOKE;

    // pub const WIDTH: i32 = 1280;
    // pub const HEIGHT: i32 = 720;
    // pub const WIDTH: i32 = 960;
    // pub const HEIGHT: i32 = 540;
    pub const WIDTH: i32 = 640;
    pub const HEIGHT: i32 = 360;

    pub fn new() -> Self {
        Engine {
            managers: Managers {
                engine: EngineManager::new(),
                log: LogManager::new(),
                state: StateManager::new(),
                audio: AudioManager::new(),
            },
            bus: Bus::new(),
            ticks: 0,
        }
    }

    pub fn run(&mut self, mut rh: &mut RaylibHandle, rt: &RaylibThread) {
        // use a render texture instead of drawing directly to screen,
        // this is to support different resolutions
        // TODO: make sure the aspect ratio is the same as the screen resolution,
        // otherwise we should draw black borders,
        // game aspect ratio is 16:9
        let mut rrt = match rh.load_render_texture(rt, Self::WIDTH as u32, Self::HEIGHT as u32) {
            Ok(rrt) => rrt,
            Err(e) => {
                panic!("Could not create render texture: {}", e);
            }
        };

        // smooths things out
        // rrt.set_texture_filter(rt, TextureFilter::TEXTURE_FILTER_BILINEAR);

        // initialize everything before we start
        self.init();

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

            // delta is used to smooth interpolation
            let delta = accumulator / self.managers.engine.size;

            // draw as often as possible
            self.draw(rt, &mut rh, &mut rrt, delta);
        }
    }

    fn init(&mut self) {
        // set the ticks per second to 16 by default
        self.bus.send(Message::Request(RequestMessage::Engine(
            EngineRequestMessage::SetTicksPerSecond(16),
        )));

        // disable debug mode by default
        self.bus.send(Message::Request(RequestMessage::Engine(
            EngineRequestMessage::SetDebug(false),
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

        self.managers
            .audio
            .update(self.bus.with_sender(Sender::Audio));

        self.bus.update(&mut self.managers);
    }

    fn draw(
        &mut self,
        rt: &RaylibThread,
        mut rh: &mut RaylibHandle,
        rrt: &mut RenderTexture2D,
        delta: f32,
    ) {
        {
            // draw everything on the render texture
            let mut rrh = rh.begin_texture_mode(rt, rrt);
            rrh.clear_background(Color::BLACK);

            self.managers.state.draw(&mut rrh, delta);

            // if self.managers.engine.debug {
            if true {
                // TODO: calc current tps
                let strings = [
                    &format!("tps {} {} {}", 16, self.managers.engine.tps, self.ticks),
                    &format!("fps {}", rrh.get_fps()),
                    &format!("dbg {}", self.managers.engine.debug),
                ];

                let mut y = 4;
                for string in strings {
                    rrh.draw_text(string, 4, y, 10, Engine::DEBUG_TEXT_COLOR);
                    y += 10;
                }

                if self.managers.engine.debug {
                    self.managers.log.draw(&mut rrh, delta);
                }
            }
        }

        // scale and draw the render texture
        let mut rdh = rh.begin_drawing(rt);

        rdh.clear_background(Color::WHITE);

        // render texture must be y-flipped due to default OpenGL coordinates (left-bottom)
        rdh.draw_texture_pro(
            &rrt,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: rrt.texture.width as f32,
                height: -rrt.texture.height as f32,
            },
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: rdh.get_screen_width() as f32,
                height: rdh.get_screen_height() as f32,
            },
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,
        );
    }
}
