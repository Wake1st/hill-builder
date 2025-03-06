mod dev;
mod draining;
mod environment;
mod flying_camera;
mod grid;
mod ground;
mod map;
mod mesh;
mod neighborhood;
mod pair;
mod selection;
mod shifting;
mod water;

use bevy::prelude::*;
use dev::DevPlugin;
use draining::DrainingPlugin;
use environment::EnvironmentPlugin;
use flying_camera::FlyingCameraPlugin;
use grid::GridPlugin;
use ground::GroundPlugin;
use map::MapPlugin;
use selection::SelectionPlugin;
use shifting::ShiftPlugin;
use water::WaterPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins).add_plugins((
            DevPlugin,
            EnvironmentPlugin,
            SelectionPlugin,
            MapPlugin,
            FlyingCameraPlugin,
            GroundPlugin,
            GridPlugin,
            ShiftPlugin,
            WaterPlugin,
            DrainingPlugin,
        ));
    }
}
