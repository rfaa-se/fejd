extern crate toml;
use std::fs::{self};
use toml::Value;

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

fn get_resolution() -> (i32, i32) {
    let toml_content = fs::read_to_string("config.toml");

    match toml_content {
        Ok(parsed) => {
            //let parsed_toml = parsed.parse::<Value>();
            match parsed.parse::<Value>() {
                Ok(parsed_toml) => {
                    // Access variables from the TOML
                    let width = parsed_toml["resolution"]["width"].as_integer().unwrap();
                    let height = parsed_toml["resolution"]["height"].as_integer().unwrap();

                    return (width.try_into().unwrap(), height.try_into().unwrap());
                }
                Err(_) => println!("Failed to parse config file."),
            }
        }
        Err(_) => println!("Failed to read config file, defaulting to 1280x720 resolition"),
    }

    (1280, 768)
}

fn main() {
    let (width, height) = get_resolution();
    let (mut rh, rt) = raylib::init().size(width, height).title("skoj").build();
    let mut engine = Engine::new();
    engine.run(&mut rh, &rt);
}
