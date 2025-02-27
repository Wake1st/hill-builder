mod block;
mod environment;
mod instructions;
mod map;
mod mesh;
mod selection;
mod shifting;

use bevy::prelude::*;
use block::BlockPlugin;
use environment::EnvironmentPlugin;
use instructions::InstructionsPlugin;
use map::MapPlugin;
use selection::SelectionPlugin;
use shifting::ShiftPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins).add_plugins((
            EnvironmentPlugin,
            SelectionPlugin,
            MapPlugin,
            BlockPlugin,
            ShiftPlugin,
            InstructionsPlugin,
        ));
    }
}
