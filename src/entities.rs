use raylib::prelude::Vector2;

use crate::{
    components::logic::{Body, Motion},
    components::render::{RenderRectangle, RenderTriangle, Renderable},
    math::{Flint, FlintRectangle, FlintTriangle, FlintVec2},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntityTypeIndex {
    Triship(usize),
    Projectile(usize),
    // Particle(usize),
}

pub struct Entities {
    pub players: Vec<Triship>, // TODO: should be triships
    pub projectiles: Vec<Projectile>,
    pub stars: Vec<Star>,
    pub exhausts: Vec<Particle>,
    pub explosions: Vec<Particle>,
}

pub struct Triship {
    pub body: Body<FlintTriangle>,
    pub motion: Motion,
    pub render: Renderable<RenderTriangle>,
    pub dead: bool,
    pub life: Flint,
}

pub struct Projectile {
    pub body: Body<FlintRectangle>,
    pub motion: Motion,
    pub render: Renderable<RenderRectangle>,
    pub dead: bool,
    pub pid: usize,
    pub dmg: Flint,
}

pub struct Particle {
    pub body: Body<FlintVec2>,
    pub motion: Motion,
    pub lifetime: i32,
    pub render: Renderable<Vector2>,
    pub dead: bool,
    pub amount: u8, // TODO: naming...
}

pub struct Star {
    pub body: Body<FlintRectangle>,
    pub render: Renderable<RenderRectangle>,
    pub counter: u8,
    pub toggle: bool,
    pub amount: u8, // TODO: naming
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            players: Vec::new(),
            projectiles: Vec::new(),
            stars: Vec::new(),
            exhausts: Vec::new(),
            explosions: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.players.clear();
        self.projectiles.clear();
        self.stars.clear();
        self.exhausts.clear();
        self.explosions.clear();
    }

    pub fn count(&self) -> usize {
        self.players.len()
            + self.projectiles.len()
            + self.stars.len()
            + self.exhausts.len()
            + self.explosions.len()
    }
}
