use engine::Engine;

mod bus;
mod commands;
mod engine;
mod entities;
mod logs;
mod math;
mod messages;
mod states;
mod world;

fn main() {
    let (mut rh, rt) = raylib::init().size(1280, 720).title("fejd").build();
    let mut engine = Engine::new();
    engine.run(&mut rh, &rt);
}
