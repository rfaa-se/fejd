use crate::{
    components::Body,
    math::{Flint, FlintTriangle},
    renderables::{RenderTriangle, Renderable},
};

pub struct Entities {
    pub players: Vec<Player>,
}

pub struct Player {
    pub body: Body<FlintTriangle>,
    pub rotation_speed: Flint,
    pub render: Renderable<RenderTriangle>,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            players: Vec::new(),
        }
    }
}
