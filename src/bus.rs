use std::collections::VecDeque;

use crate::{
    engine::Managers,
    messages::{Message, Sender},
};

pub struct Bus {
    messages: VecDeque<(Sender, Message)>,
    current_sender: Sender,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            messages: VecDeque::new(),
            current_sender: Sender::None,
        }
    }

    pub fn update(&mut self, managers: &mut Managers) {
        while let Some((sender, msg)) = self.messages.pop_front() {
            managers.log.message(&sender, &msg);
            managers.engine.message(&sender, &msg);
            managers.state.message(&sender, &msg);
        }
    }

    pub fn send(&mut self, msg: Message) {
        self.messages.push_back((self.current_sender, msg));
    }

    pub fn with_sender(&mut self, sender: Sender) -> &mut Self {
        self.current_sender = sender;
        self
    }
}
