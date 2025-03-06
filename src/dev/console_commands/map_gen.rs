use std::str::{FromStr, Split};
use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand};
use clap::Parser;

use crate::map::{ClearMap, CurvedTerrainSettings, GenerateMap, MapGenerationSettings, TerrainSettings};

const HELP_REPLY: &str = "\tgenerate args:
\nsize.(i32) - sets the map size
\nterrain.(type)[.(curve param)] - sets the shape of the map";
const TERRAIN_HELP_REPLY: &str = "\tterrain args:
\nflat: for flat terrain
\ncurved: for curved terrain (curve params):
\namp: amplitude
\nwave: wavelength
\nvert: vertical shift
\nphase: phase shift";

pub struct MapGenCommandsPlugin;

impl Plugin for MapGenCommandsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_console_command::<RemoveMapCommand, _>(clear_command)
            .add_console_command::<GenerateMapCommand, _>(generate_command);
    }
}

/// clear map command
#[derive(Parser, ConsoleCommand)]
#[command(name = "remove")]
struct RemoveMapCommand;

fn clear_command(
    mut log: ConsoleCommand<RemoveMapCommand>,
    mut cleanup: EventWriter<ClearMap>
) {
    if let Some(Ok(RemoveMapCommand)) = log.take() {
        cleanup.send(ClearMap);
    }
}

/// generate map command
#[derive(Parser, ConsoleCommand)]
#[command(name = "generate")]
struct GenerateMapCommand {
    /// Some message
    args: String,
}

fn generate_command(
    mut log: ConsoleCommand<GenerateMapCommand>,
    mut generator: EventWriter<GenerateMap>
) {
    if let Some(Ok(GenerateMapCommand { args })) = log.take() {
        let lowercase = args.to_lowercase();
        let arg_parts = lowercase.split("|");
        let mut map_settings = MapGenerationSettings::default(); 

        for part in arg_parts {
            let mut sub_command = part.split("-");

            match sub_command.next() {
                Some("help") => {
                    log.reply(HELP_REPLY);
                    return;
                },
                Some("size") => {
                    let Some(map_size) = parse_sub_command::<i32>(sub_command.next()) else {
                        log.reply("error (map size): could not parse the size provided - please use i32.");
                        continue;
                    };
                    map_settings.size = map_size;
                },
                Some("terrain") => {
                    let Some(terrain_type) = sub_command.next() else {
                        log.reply(format!("error (terrain type): no terrain type provided."));
                        return;
                    };

                    match terrain_type {
                        "flat" => map_settings.terrain = TerrainSettings::FLAT,
                        "curved" => map_settings.terrain = TerrainSettings::CURVED(parse_curved_terrain_args(&mut sub_command)),
                        "help" => log.reply(TERRAIN_HELP_REPLY),
                        _ => {
                            log.reply("error (terrain type): terrain type not recognized.");
                            return;
                        },
                    };
                    log.reply(format!("\tgenerating {:?} terrain.", terrain_type));
                },
                Some(p) => {
                    log.reply(format!("error (args): {:?} does not match any known arg.", p));
                    return;
                },
                None => {
                    log.reply(format!("error (args): cannot could not comprehend arg."));
                    return;
                } 
            }
        }

        generator.send(GenerateMap {
            settings: map_settings,
        });
    }
}

fn parse_curved_terrain_args(sub_command: &mut Split<'_, &str>) -> CurvedTerrainSettings {
    let mut settings = CurvedTerrainSettings::default(); 

    loop {
        let Some(sub) = sub_command.next() else {
            break;
        };

        let Some(x_val) = parse_sub_command::<f32>(sub_command.next()) else {
            sub_command.next();
            break;
        };
    
        let Some(y_val) = parse_sub_command::<f32>(sub_command.next()) else {
            break;
        };
        let vec = Vec2::new(x_val, y_val);
    
        match sub {
            "amp" => settings.amplitude = vec,
            "wave" => settings.wavelength = vec,
            "vert" => settings.vertical_shift = vec,
            "phase" => settings.phase_shift = vec,
            _ => ()
        }
    }

    return settings;
}

fn parse_sub_command<T>(option: Option<&str>) -> Option<T>
where T: FromStr 
{
    let Some(str) = option else {
        return None;
    };
    let Ok(val) = str.parse::<T>() else {
        return None;
    };
    return Some(val);
}
