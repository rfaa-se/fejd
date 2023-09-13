use raylib::prelude::*;

use crate::{
    components::Body,
    math::{Flint, FlintTriangle},
};

pub struct Entities {
    pub players: Vec<Player>,
}

pub struct Player {
    pub color: Color,
    pub body: Body<FlintTriangle>,
    pub rotation_speed: Flint,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            players: Vec::new(),
        }
    }
}
