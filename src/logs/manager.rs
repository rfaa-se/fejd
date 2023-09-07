use std::collections::VecDeque;

use crate::{
    bus::Bus,
    messages::{Message, Sender},
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
}
