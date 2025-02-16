use bevy::{prelude::*, render::camera::ScalingMode};

const CAMERA_DISTANCE: f32 = 6.0;

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
        Transform::from_xyz(CAMERA_DISTANCE, CAMERA_DISTANCE/2.0, CAMERA_DISTANCE)
        .looking_at(Vec3::ZERO, Vec3::Y)
    ));

    // Light up the scene.
    commands.spawn((
        PointLight {
            range: 100.0, 
            ..default()
        }, 
        Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0)
    ));

}