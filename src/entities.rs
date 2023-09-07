use raylib::prelude::*;

use crate::{components::Body, math::FlintTriangle};

pub struct Entities {
    pub players: Vec<Player>,
}

pub struct Player {
    pub color: Color,
    pub position: Body<FlintTriangle>,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            players: Vec::new(),
        }
    }
}
