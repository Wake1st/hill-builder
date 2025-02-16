use bevy::prelude::*;

use crate::block::Block;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<StoreCursor>()
            .add_systems(Update, cursor_update);
    }
}

fn cursor_update(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    blocks: Query<&GlobalTransform, With<Block>>,
    windows: Single<&Window>,
    mut gizmos: Gizmos,
    mut buttons: ResMut<ButtonInput<MouseButton>>,
    mut store_cursor: EventWriter<StoreCursor>
) {
    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Iterate through the ground pieces
    for transform in blocks.iter() {
        // Calculate if and where the ray is hitting the ground plane.
        let Some(distance) = ray.intersect_plane(
            transform.translation() + Vec3::new(0.0,0.5,0.0), 
            InfinitePlane3d::new(transform.up())
        ) else {
            return;
        };
        let point: Vec3 = ray.get_point(distance);

        // Store cursor position
        if buttons.clear_just_pressed(MouseButton::Left) {
            store_cursor.send(StoreCursor { position: point.xz(), pull: true });
        }
        if buttons.clear_just_pressed(MouseButton::Right) {
            store_cursor.send(StoreCursor { position: point.xz(), pull: false });
        }

        // Draw a circle just above the ground plane at that position.
        gizmos.circle(
            Isometry3d::new(
                point + transform.up() * 0.01,
                Quat::from_rotation_arc(Vec3::Z, transform.up().as_vec3()),
            ),
            0.2,
            Color::WHITE,
        );
    }
}

#[derive(Event)]
pub struct StoreCursor {
    pub position: Vec2,
    pub pull: bool,
}
