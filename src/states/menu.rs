use std::collections::BTreeSet;

use raylib::prelude::*;

use crate::{
    bus::Bus,
    messages::{Message, RequestMessage, Sender, StateRequestMessage},
    misc::RaylibRenderHandle,
};

use super::State;

pub struct MenuState {
    actions: BTreeSet<Action>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    GotoGame,
}

impl MenuState {
    pub fn new() -> Self {
        MenuState {
            actions: BTreeSet::new(),
        }
    }

    pub fn init(&mut self) {}

    pub fn exit(&mut self) {
        self.actions.clear();
    }

    pub fn input(&mut self, rh: &RaylibHandle) {
        if rh.is_key_pressed(KeyboardKey::KEY_S) {
            self.actions.insert(Action::GotoGame);
        }
    }

    pub fn update(&mut self, bus: &mut Bus) {
        self.action(bus);
    }

    pub fn message(&mut self, _sender: &Sender, _msg: &Message) {}

    pub fn draw(&mut self, _rrh: &mut RaylibRenderHandle, _delta: f32) {}

    fn action(&mut self, bus: &mut Bus) {
        while let Some(action) = self.actions.pop_first() {
            match action {
                Action::GotoGame => {
                    bus.send(Message::Request(RequestMessage::State(
                        StateRequestMessage::SetState(State::Game),
                    )));
                }
            }
        }
    }
}
