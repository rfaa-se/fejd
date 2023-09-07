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
mod states;
mod world;

fn get_resolution() -> (i32, i32) {
    let toml_content = fs::read_to_string("config.toml");

    match toml_content {
        Ok(parsed) => match parsed.parse::<Value>() {
            Ok(parsed_toml) => {
                let width = parsed_toml["resolution"]["width"].as_integer();
                let height = parsed_toml["resolution"]["height"].as_integer();

                match (width, height) {
                    (Some(width), Some(height)) => {
                        let width: Result<i32, _> = width.try_into();
                        let height: Result<i32, _> = height.try_into();

                        match (width, height) {
                            (Ok(width), Ok(height)) => return (width, height),
                            _ => println!("Failed to parse resolution"),
                        }
                    }
                    _ => println!("Failed to parse resolution"),
                }
            }
            Err(_) => println!("Failed to parse config file."),
        },
        Err(_) => println!("Failed to read config file, defaulting to 1280x720 resolition"),
    }

    (1280, 768)
}

fn main() {
    let (width, height) = get_resolution();
    let (mut rh, rt) = raylib::init().size(width, height).title("fejd").build();
    let mut engine = Engine::new();
    engine.run(&mut rh, &rt);
}
