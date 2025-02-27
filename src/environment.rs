use bevy::prelude::*;

const LIGHT_DISTANCE: f32 = 32.0;
const LIGHT_RANGE: f32 = 1024.;
const LIGHT_INTENSITY: f32 = 4_000_000.;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    // Light up the scene.
    commands.spawn((
        PointLight {
            range: LIGHT_RANGE,
            intensity: LIGHT_INTENSITY,
            ..default()
        },
        Transform::from_xyz(0.0, LIGHT_DISTANCE, 0.0),
    ));
}
