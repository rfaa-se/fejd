use std::collections::BTreeSet;

use raylib::prelude::*;

use crate::{
    bus::Bus,
    messages::{
        EngineMessage, EngineRequestMessage, Message, RequestMessage, Sender, StateRequestMessage,
    },
    misc::RaylibRenderHandle,
};

use super::State;

pub struct MenuState {
    actions: BTreeSet<Action>,
    debug: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    GotoGame,
    GetDebug,
    ToggleDebug,
}

impl MenuState {
    pub fn new() -> Self {
        MenuState {
            actions: BTreeSet::new(),
            debug: false,
        }
    }

    pub fn init(&mut self) {
        self.actions.insert(Action::GetDebug);
    }

    pub fn exit(&mut self) {
        self.actions.clear();
    }

    pub fn input(&mut self, rh: &RaylibHandle) {
        if rh.is_key_pressed(KeyboardKey::KEY_S) {
            self.actions.insert(Action::GotoGame);
        }

        if rh.is_key_pressed(KeyboardKey::KEY_D) {
            self.actions.insert(Action::ToggleDebug);
        }
    }

    pub fn update(&mut self, bus: &mut Bus) {
        self.action(bus);
    }

    pub fn message(&mut self, _sender: &Sender, msg: &Message) {
        match msg {
            Message::Engine(EngineMessage::DebugSet(debug) | EngineMessage::DebugGet(debug)) => {
                self.debug = *debug;
            }
            _ => return,
        }
    }

    pub fn draw(&mut self, _rrh: &mut RaylibRenderHandle, _delta: f32) {}

    fn action(&mut self, bus: &mut Bus) {
        while let Some(action) = self.actions.pop_first() {
            match action {
                Action::GotoGame => {
                    bus.send(Message::Request(RequestMessage::State(
                        StateRequestMessage::SetState(State::Game),
                    )));
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
            }
        }
    }
}
