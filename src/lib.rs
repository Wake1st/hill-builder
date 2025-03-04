mod block;
mod environment;
mod flying_camera;
mod instructions;
mod map;
mod mesh;
mod selection;
mod shifting;
mod console_commands;
mod water;
mod draining;

use bevy::prelude::*;
use block::BlockPlugin;
use console_commands::ConComPlugin;
use draining::DrainingPlugin;
use environment::EnvironmentPlugin;
use flying_camera::FlyingCameraPlugin;
use instructions::InstructionsPlugin;
use map::MapPlugin;
use selection::SelectionPlugin;
use shifting::ShiftPlugin;
use water::WaterPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins).add_plugins((
            ConComPlugin,
            EnvironmentPlugin,
            SelectionPlugin,
            MapPlugin,
            FlyingCameraPlugin,
            BlockPlugin,
            ShiftPlugin,
            WaterPlugin,
            DrainingPlugin,
            InstructionsPlugin,
        ));
    }
}
