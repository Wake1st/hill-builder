use bevy::{prelude::*, render::camera::ScalingMode};

const CAMERA_DISTANCE: f32 = 28.0;
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
    // Camera in 3D space.
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            // 6 world units per pixel of window height.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: CAMERA_DISTANCE,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(CAMERA_DISTANCE, CAMERA_DISTANCE * 0.8, CAMERA_DISTANCE)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

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
