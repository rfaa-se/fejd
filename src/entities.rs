use crate::{
    components::logic::{Body, Motion},
    components::render::{RenderRectangle, RenderTriangle, Renderable},
    math::{FlintRectangle, FlintTriangle},
};

pub struct Entities {
    pub players: Vec<Triship>,
    pub projectiles: Vec<Projectile>,
}

pub struct Triship {
    pub body: Body<FlintTriangle>,
    pub motion: Motion,
    pub render: Renderable<RenderTriangle>,
    pub dead: bool,
}

pub struct Projectile {
    pub body: Body<FlintRectangle>,
    pub motion: Motion,
    pub render: Renderable<RenderRectangle>,
    pub dead: bool,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            players: Vec::new(),
            projectiles: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.players.clear();
        self.projectiles.clear();
    }

    pub fn get_count(&self) -> usize {
        self.players.len() + self.projectiles.len()
    }
}
