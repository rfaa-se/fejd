use std::collections::BTreeSet;

use crate::{
    bus::Bus,
    entities::EntityTypeIndex,
    messages::{AudioMessage, LogicMessage, Message, Sender},
};

pub struct AudioManager {
    actions: BTreeSet<Action>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    Play(EntityTypeIndex),
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            actions: BTreeSet::new(),
        }
    }

    pub fn update(&mut self, bus: &mut Bus) {
        self.action(bus);
    }

    pub fn message(&mut self, _sender: &Sender, msg: &Message) {
        if let Message::Logic(LogicMessage::Death(idx)) = msg {
            if let EntityTypeIndex::Triship(_) = idx {
                // we only want to play sounds for triship deaths for now
                self.actions.insert(Action::Play(*idx));
            }
        }
    }

    fn action(&mut self, bus: &mut Bus) {
        while let Some(action) = self.actions.pop_first() {
            match action {
                Action::Play(idx) => {
                    // TODO: play death sound!
                    // TODO: we might want to move the world/entities out of the game state,
                    // so we can check whether the player is nearby the dead entity
                    bus.send(Message::Audio(AudioMessage::Play(idx)));
                }
            }
        }
    }
}
