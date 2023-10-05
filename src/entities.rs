use crate::{
    components::{Body, Motion},
    math::{FlintRectangle, FlintTriangle},
    renderables::{RenderRectangle, RenderTriangle, Renderable},
};

pub struct Entities {
    pub players: Vec<Player>,
    pub projectiles: Vec<Projectile>,
}

pub struct Player {
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
}
