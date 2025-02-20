use bevy::{picking::backend::ray::RayMap, prelude::*};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<StoreCursor>()
            .add_systems(Update, store_cursor_selection);
    }
}

#[derive(Event)]
pub struct StoreCursor {
    pub position: Vec2,
    pub pull: bool,
}

fn store_cursor_selection(
    mut ray_cast: MeshRayCast,
    // The ray map stores rays cast by the cursor
    ray_map: Res<RayMap>,
    mut buttons: ResMut<ButtonInput<MouseButton>>,
    mut store_cursor: EventWriter<StoreCursor>
) {
    let left_selected = buttons.clear_just_pressed(MouseButton::Left);
    let right_selected = buttons.clear_just_pressed(MouseButton::Right);
    if left_selected || right_selected {
        // Cast a ray from the cursor and bounce it off of surfaces
        for (_, ray) in ray_map.iter() {
            // Cast the ray and get the first hit
            let Some((_, hit)) = ray_cast.cast_ray(*ray, &RayCastSettings::default()).first() else {
                break;
            };
            
            // Store cursor position
            store_cursor.send(StoreCursor { position: hit.point.xz(), pull: left_selected });
        }
    };
}
