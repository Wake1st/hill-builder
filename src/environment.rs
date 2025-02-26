use bevy::prelude::*;

const LIGHT_DISTANCE: f32 = 6.0;
const LIGHT_RANGE: f32 = 100.;

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
            ..default()
        },
        Transform::from_xyz(0.0, LIGHT_DISTANCE, 0.0),
    ));
}
