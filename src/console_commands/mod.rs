pub mod map_gen;
mod map_file;

use bevy::prelude::*;
use bevy_console::ConsolePlugin;
use map_file::MapFileCommandsPlugin;
use map_gen::MapGenCommandsPlugin;

pub struct ConComPlugin;

impl Plugin for ConComPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ConsolePlugin, MapGenCommandsPlugin, MapFileCommandsPlugin));
    }
}
