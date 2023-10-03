use crate::{
    components::{Body, Motion},
    math::FlintTriangle,
    renderables::{RenderTriangle, Renderable},
};

pub struct Entities {
    pub players: Vec<Player>,
}

pub struct Player {
    pub body: Body<FlintTriangle>,
    pub motion: Motion,
    pub render: Renderable<RenderTriangle>,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            players: Vec::new(),
        }
    }
}
