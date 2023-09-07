use std::collections::BTreeSet;

use crate::{
    bus::Bus,
    messages::{EngineMessage, EngineRequestMessage, Message, RequestMessage, Sender},
};

pub struct EngineManager {
    pub(super) tps: u8,
    pub(super) size: f32,
    pub(super) debug: bool,
    actions: BTreeSet<Action>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    SetTicksPerSecond(u8),
    SetDebug(bool),
}

impl EngineManager {
    pub fn new() -> Self {
        EngineManager {
            tps: 0,
            size: 0.0,
            debug: false,
            actions: BTreeSet::new(),
        }
    }

    pub fn update(&mut self, bus: &mut Bus) {
        self.action(bus);
    }

    pub fn message(&mut self, _sender: &Sender, msg: &Message) {
        // we only care about engine requests
        let req = match msg {
            Message::Request(RequestMessage::Engine(msg)) => msg,
            _ => return,
        };

        match req {
            EngineRequestMessage::SetTicksPerSecond(tps) => {
                self.actions.insert(Action::SetTicksPerSecond(*tps));
            }
            EngineRequestMessage::SetDebug(debug) => {
                self.actions.insert(Action::SetDebug(*debug));
            }
        }
    }

    fn action(&mut self, bus: &mut Bus) {
        while let Some(action) = self.actions.pop_first() {
            match action {
                Action::SetTicksPerSecond(tps) => {
                    self.tps = tps;
                    self.size = 1.0 / tps as f32;

                    bus.send(Message::Engine(EngineMessage::TicksPerSecondSet(tps)));
                }
                Action::SetDebug(debug) => {
                    self.debug = debug;

                    bus.send(Message::Engine(EngineMessage::DebugSet(debug)));
                }
            }
        }
    }
}
