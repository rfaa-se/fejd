use std::collections::VecDeque;

use raylib::prelude::RaylibDraw;

use crate::{
    bus::Bus,
    engine::Engine,
    messages::{Message, Sender},
    misc::RaylibRenderHandle,
};

pub struct LogManager {
    logs: VecDeque<String>,
}

impl LogManager {
    pub fn new() -> Self {
        LogManager {
            logs: VecDeque::new(),
        }
    }

    pub fn update(&mut self, _bus: &mut Bus) {}

    pub fn message(&mut self, sender: &Sender, msg: &Message) {
        let log = format!("{:?} | {:?}", sender, msg);

        println!("INFO: MSG: {}", log);

        // we only save the latest 20 logs
        if self.logs.len() > 20 {
            self.logs.pop_front();
        }

        self.logs.push_back(log);
    }

    pub fn draw(&self, rrh: &mut RaylibRenderHandle, _delta: f32) {
        self.logs.iter().fold(
            Engine::HEIGHT - 4 - self.logs.len() as i32 * 10,
            |y, log| {
                rrh.draw_text(log, 4, y, 10, Engine::DEBUG_TEXT_COLOR);
                y + 10
            },
        );
    }
}
