mod block;
mod cursor;
mod environment;
mod flying_camera;
mod instructions;
mod mesh;
mod shifting;

use bevy::prelude::*;
use block::BlockPlugin;
use cursor::CursorPlugin;
use environment::EnvironmentPlugin;
use flying_camera::FlyingCameraPlugin;
use instructions::InstructionsPlugin;
use shifting::ShiftPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins).add_plugins((
            CursorPlugin,
            FlyingCameraPlugin,
            EnvironmentPlugin,
            BlockPlugin,
            ShiftPlugin,
            InstructionsPlugin,
        ));
    }
}
