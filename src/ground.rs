use bevy::prelude::*;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Ground;
