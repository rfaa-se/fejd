use raylib::prelude::Vector2;

use crate::{
    components::logic::{Body, Motion},
    components::render::{RenderRectangle, RenderTriangle, Renderable},
    math::{FlintRectangle, FlintTriangle, FlintVec2},
};

pub struct Entities {
    pub players: Vec<Triship>,
    pub projectiles: Vec<Projectile>,
    pub particles: Vec<Particle>,
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
    pub pid: usize,
}

pub struct Particle {
    pub body: Body<FlintVec2>,
    pub motion: Motion,
    pub lifetime: i32,
    pub render: Renderable<Vector2>,
    pub dead: bool,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            players: Vec::new(),
            projectiles: Vec::new(),
            particles: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.players.clear();
        self.projectiles.clear();
        self.particles.clear();
    }

    pub fn get_count(&self) -> usize {
        self.players.len() + self.projectiles.len() + self.particles.len()
    }
}
