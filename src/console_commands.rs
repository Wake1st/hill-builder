use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use clap::Parser;

const ARG_HELP: &str = "help";
const ARG_TERRAIN: &str = "terrain";
const ARG_TERRAIN_FLAT: &str = "flat";
const ARG_TERRAIN_CURVED: &str = "curved";

const HELP_REPLY: &str = "\tgenerate args:\n--terrain: sets the shape of the map";

pub struct ConComPlugin;

impl Plugin for ConComPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConsolePlugin)
            .add_console_command::<GenerateMapCommand, _>(example_command);
    }
}

/// generate map command
#[derive(Parser, ConsoleCommand)]
#[command(name = "generate")]
struct GenerateMapCommand {
    /// Some message
    msg: String,
}

fn example_command(mut log: ConsoleCommand<GenerateMapCommand>) {
    if let Some(Ok(GenerateMapCommand { msg })) = log.take() {
        let lowercase = msg.to_lowercase();
        let arg_parts = lowercase.split("--");

        for part in arg_parts {
            let mut sub_command = part.split(" ");
            match sub_command.next() {
                Some(ARG_HELP) => log.reply(HELP_REPLY),
                Some(ARG_TERRAIN) => {
                    let Some(terrain_type) = sub_command.next() else {
                        log.reply(format!("ERROR: no terrain type provided."));
                        return;
                    };

                    log.reply(format!("\tgenerating {:?} terrain.", terrain_type));
                    match terrain_type {
                        ARG_TERRAIN_FLAT => (),
                        ARG_TERRAIN_CURVED => (),
                        _ => ()
                    }
                },
                Some(p) => log.reply(format!("ERROR: {:?} does not match any known arg.", p)),
                None => log.reply(format!("ERROR: cannot could not comprehend arg.")), 
            }
        }
    }
}