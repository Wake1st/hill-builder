use std::{fs::{self, File}, io::Write};

use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand};
use clap::Parser;

use crate::map::{GenerateMap, CurrentMapSettings};

pub struct MapFileCommandsPlugin;

impl Plugin for MapFileCommandsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_console_command::<SaveMapCommand, _>(save_map_command)
            .add_console_command::<LoadMapCommand, _>(load_map_command);
    }
}

/// save current map to file
#[derive(Parser, ConsoleCommand)]
#[command(name = "save-map")]
struct SaveMapCommand {
    /// file name
    name: String,
}

fn save_map_command(mut log: ConsoleCommand<SaveMapCommand>, settings: Res<CurrentMapSettings>) {
    if let Some(Ok(SaveMapCommand { name })) = log.take() {
        let json_data = match serde_json::to_string_pretty(&settings.value) {
            Err(e) => panic!("error: failed to serialize map settings: {:?}", e),
            Ok(data) => data,
        };

        let path = format!("./assets/maps/{}.json", name);
        let mut file = match File::create(path) {
            Err(e) => panic!("error: failed to create file: {:?}", e),
            Ok(data) => data,
        };

        match file.write_all(json_data.as_bytes()) {
            Err(e) => panic!("error: failed to write data to file: {:?}", e),
            Ok(_) => (),
        };
    }
}

/// generate a map from file data
#[derive(Parser, ConsoleCommand)]
#[command(name = "load-map")]
struct LoadMapCommand {
    /// file name
    name: String,
}

fn load_map_command(mut log: ConsoleCommand<LoadMapCommand>, mut generate: EventWriter<GenerateMap>) {
    if let Some(Ok(LoadMapCommand { name })) = log.take() {
        let path = format!("./assets/maps/{}.json", name);
        let json_data = match fs::read_to_string(path) {
            Err(e) => panic!("error: failed to read data from file: {:?}", e),
            Ok(data) => data,
        };

        match serde_json::from_str(&json_data) {
            Err(e) => panic!("error: failed to deserialize file data: {:?}", e),
            Ok(settings) => { generate.send(GenerateMap { settings }); },
        }
    }
}