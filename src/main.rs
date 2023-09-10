extern crate toml;
use std::fs::{self};
use toml::Value;

use engine::Engine;

mod bus;
mod commands;
mod components;
mod engine;
mod entities;
mod logs;
mod math;
mod messages;
mod misc;
mod states;
mod world;

fn main() {
    let (width, height) = get_resolution();
    let (mut rh, rt) = raylib::init().size(width, height).title("fejd").build();
    let mut engine = Engine::new();
    engine.run(&mut rh, &rt);
}

fn get_resolution() -> (i32, i32) {
    // TODO: if we add more configs, consider moving parsing file to its own function
    let default = (1280, 720);

    let config = match fs::read_to_string("config.toml") {
        Ok(config) => config,
        Err(_) => {
            println!("Failed to read config.toml file");
            return default;
        }
    };

    let config = match config.parse::<Value>() {
        Ok(config) => config,
        Err(_) => {
            println!("Failed to parse config.toml file");
            return default;
        }
    };

    let (width, height) = (
        config["resolution"]["width"].as_integer(),
        config["resolution"]["height"].as_integer(),
    );

    let (width, height) = match (width, height) {
        (Some(width), Some(height)) => (width.try_into(), height.try_into()),
        _ => {
            println!("Failed to parse resolution");
            return default;
        }
    };

    match (width, height) {
        (Ok(width), Ok(height)) => (width, height),
        _ => {
            println!("Failed to read resolution");
            default
        }
    }
}
