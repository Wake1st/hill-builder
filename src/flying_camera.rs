use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::ScalingMode,
};

const CAMERA_DISTANCE: f32 = 24.0;

const CAMERA_ZOOM_RATE: f32 = 6.0;
const CAMERA_ROTATION_SENSITIVITY: f32 = 0.06;
const CAMERA_ZOOM_MIN: f32 = 0.5;
const CAMERA_ZOOM_MAX: f32 = 2.0;
const CAMERA_TRANSLATE_RATE: f32 = 12.0;

pub struct FlyingCameraPlugin;

impl Plugin for FlyingCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraDirection::default());

        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                (keyboard_input, mouse_motion, scrollwheel_input),
                move_camera,
            )
                .chain(),
        );
    }
}

#[derive(Component, Default, Debug)]
pub struct CameraRoot;

#[derive(Component, Default, Debug)]
pub struct FlyingCamera;

#[derive(Resource, Default, Debug)]
pub struct CameraDirection {
    pub translation: Vec2,
    pub rotation: Vec2,
    pub zoom: f32,
}

fn setup(mut commands: Commands) {
    // Camera in 3D space.
    commands
        .spawn((CameraRoot, Transform::default(), Name::new("camera root")))
        .with_children(|parent| {
            parent.spawn((
                FlyingCamera,
                Camera3d::default(),
                Projection::from(OrthographicProjection {
                    // 6 world units per pixel of window height.
                    scaling_mode: ScalingMode::FixedVertical {
                        viewport_height: CAMERA_DISTANCE,
                    },
                    ..OrthographicProjection::default_3d()
                }),
                Transform::from_xyz(0.0, CAMERA_DISTANCE, CAMERA_DISTANCE)
                    .looking_at(Vec3::ZERO, Vec3::Y),
            ));
        });
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut direction: ResMut<CameraDirection>) {
    let mut dir: Vec2 = Vec2::ZERO;

    if keys.any_pressed(vec![KeyCode::KeyD, KeyCode::ArrowRight].into_iter()) {
        dir.x += 1.0;
    }
    if keys.any_pressed(vec![KeyCode::KeyA, KeyCode::ArrowLeft].into_iter()) {
        dir.x -= 1.0;
    }
    if keys.any_pressed(vec![KeyCode::KeyW, KeyCode::ArrowUp].into_iter()) {
        dir.y += 1.0;
    }
    if keys.any_pressed(vec![KeyCode::KeyS, KeyCode::ArrowDown].into_iter()) {
        dir.y -= 1.0;
    }

    direction.translation = dir;
}

fn mouse_motion(
    buttons: Res<ButtonInput<MouseButton>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut direction: ResMut<CameraDirection>,
) {
    direction.rotation = Vec2::ZERO;

    if buttons.pressed(MouseButton::Middle) {
        for ev in evr_motion.read() {
            direction.rotation = ev.delta;
        }
    }
}

fn scrollwheel_input(
    mut evr_scroll: EventReader<MouseWheel>,
    mut direction: ResMut<CameraDirection>,
) {
    for ev in evr_scroll.read() {
        direction.zoom = -ev.y;
    }
}

fn move_camera(
    root: Single<(&mut Transform, &GlobalTransform), (With<CameraRoot>, Without<FlyingCamera>)>,
    camera: Single<&mut Projection, (With<FlyingCamera>, Without<CameraRoot>)>,
    direction: Res<CameraDirection>,
    time: Res<Time>,
) {
    //  get values
    let (mut root_transform, global_root_transform) = root.into_inner();
    let mut camera_projection = camera.into_inner();
    let delta_time: f32 = time.delta_secs();

    //  set the zoom
    let delta_scale = 1. + direction.zoom * CAMERA_ZOOM_RATE * delta_time;
    match *camera_projection {
        Projection::Orthographic(ref mut orthographic) => {
            orthographic.scale =
                (orthographic.scale * delta_scale).clamp(CAMERA_ZOOM_MIN, CAMERA_ZOOM_MAX)
        }
        _ => (),
    };

    //  rotate with the mouse
    root_transform.rotate_y(direction.rotation.x * CAMERA_ROTATION_SENSITIVITY);

    //  we must move the camera root relative to the facing direction of the camera
    let relative_direction = global_root_transform.rotation()
        * Vec3::new(direction.translation.x, 0.0, -direction.translation.y);
    root_transform.translation += relative_direction * CAMERA_TRANSLATE_RATE * delta_time;
}
