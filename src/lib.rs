mod mesh;
mod cursor;
mod block;
mod instructions;
mod environment;
mod shifting;

use bevy::prelude::*;
use block::BlockPlugin;
use cursor::CursorPlugin;
use environment::EnvironmentPlugin;
use instructions::InstructionsPlugin;
use shifting::ShiftPlugin;


pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
            .add_plugins((CursorPlugin, EnvironmentPlugin, BlockPlugin, ShiftPlugin, InstructionsPlugin));
    }
}
