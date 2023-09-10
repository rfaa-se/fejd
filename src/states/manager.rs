use std::collections::BTreeSet;

use raylib::prelude::*;

use crate::{
    bus::Bus,
    messages::{Message, RequestMessage, Sender, StateMessage, StateRequestMessage},
    misc::RaylibRenderHandle,
};

use super::{GameState, MenuState, State};

pub struct StateManager {
    current: State,
    states: States,
    actions: BTreeSet<Action>,
    debug_text: String,
    debug_text_w: i32,
}

struct States {
    menu: MenuState,
    game: GameState,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    SetState(State),
}

impl StateManager {
    pub fn new() -> Self {
        let state = State::None;
        let text = format!("{:?}", state);

        StateManager {
            current: state,
            states: States {
                menu: MenuState::new(),
                game: GameState::new(),
            },
            actions: BTreeSet::new(),
            debug_text: text.to_owned(),
            debug_text_w: raylib::text::measure_text(&text, 10),
        }
    }

    pub fn input(&mut self, rh: &RaylibHandle) {
        match self.current {
            State::None => (),
            State::Menu => self.states.menu.input(rh),
            State::Game => self.states.game.input(rh),
        }
    }

    pub fn update(&mut self, bus: &mut Bus) {
        self.action(bus);

        match self.current {
            State::None => (),
            State::Menu => self.states.menu.update(bus.with_sender(Sender::Menu)),
            State::Game => self.states.game.update(bus.with_sender(Sender::Game)),
        }
    }

    pub fn message(&mut self, sender: &Sender, msg: &Message) {
        match self.current {
            State::None => (),
            State::Menu => self.states.menu.message(sender, msg),
            State::Game => self.states.game.message(sender, msg),
        }

        // we only care about state requests
        let req = match msg {
            Message::Request(RequestMessage::State(msg)) => msg,
            _ => return,
        };

        match req {
            StateRequestMessage::SetState(state) => {
                self.actions.insert(Action::SetState(*state));
            }
        }
    }

    pub fn draw(&mut self, rrh: &mut RaylibRenderHandle, delta: f32) {
        match self.current {
            State::None => (),
            State::Menu => self.states.menu.draw(rrh, delta),
            State::Game => self.states.game.draw(rrh, delta),
        }

        // TODO: debug
        if true {
            // TODO: get width from render texture
            rrh.draw_text(
                &self.debug_text,
                rrh.get_screen_width() / 2 - self.debug_text_w / 2,
                4,
                10,
                Color::WHITESMOKE,
            );
        }
    }

    fn action(&mut self, bus: &mut Bus) {
        while let Some(action) = self.actions.pop_first() {
            match action {
                Action::SetState(state) => {
                    match self.current {
                        State::None => (),
                        State::Menu => self.states.menu.exit(),
                        State::Game => self.states.game.exit(),
                    }

                    self.current = state;
                    self.debug_text = format!("{:?}", self.current);
                    self.debug_text_w = raylib::text::measure_text(&self.debug_text, 12);

                    match self.current {
                        State::None => (),
                        State::Menu => self.states.menu.init(),
                        State::Game => self.states.game.init(),
                    }

                    bus.send(Message::State(StateMessage::StateSet(state)));
                }
            }
        }
    }
}
