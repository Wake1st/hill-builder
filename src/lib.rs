mod mesh;
mod cursor;
mod block;
mod instructions;
mod environment;

use bevy::prelude::*;
use block::BlockPlugin;
use cursor::CursorPlugin;
use environment::EnvironmentPlugin;
use instructions::InstructionsPlugin;


pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
            .add_plugins((CursorPlugin, EnvironmentPlugin, BlockPlugin, InstructionsPlugin));
    }
}
