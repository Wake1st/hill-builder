use bevy::prelude::*;

use crate::{cursor::StoreCursor, mesh::create_cube_mesh, shifting::{Shifting, SHIFT_AMOUNT}};

const GROUND_COLOR: Color = Color::srgb(0.0, 0.9, 0.1);
const MAP_SIZE: i32 = 12;
const GAP: f32 = 0.1;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_systems(Update, read_position);
}
}

#[derive(Component)]
pub struct Block {
    pub row: i32,
    pub col: i32,
    pub layer: f32,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let offset: f32 = (MAP_SIZE as f32) * GAP / 2.0;

    for j in 0..MAP_SIZE {
        for i in 0..MAP_SIZE {
            // Create and save a handle to the mesh.
            let cube_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh());
        
            // Render the mesh with the custom texture, and add the marker.
            commands.spawn((
                Mesh3d(cube_mesh_handle),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: GROUND_COLOR,
                    ..default()
                })),
                Transform::from_xyz((i as f32)*(1.0 + GAP) - offset, 0.0, (j as f32)*(1.0 + GAP) - offset),
                Block { row: i, col: j, layer: 0.0, }
            ));
        }
    }

    // Text to describe the controls.
    commands.spawn((
        Text::new("Left click a block to pull it up; right click a block to push it down."),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}


fn read_position(
    mut cursor_position: EventReader<StoreCursor>,
    mut blocks: Query<(Entity, &GlobalTransform, &mut Block)>,
    mut commands: Commands,
) {
    for cursor in cursor_position.read() {
        for (entity, transform, mut block) in blocks.iter_mut() {
            let rect = Rect::from_center_half_size(
                transform.translation().xz(), 
                Vec2::new(0.5, 0.5)
            );
            if rect.contains(cursor.position) {
                block.layer += if cursor.pull { SHIFT_AMOUNT } else { -SHIFT_AMOUNT };
                commands.entity(entity).insert(Shifting { up: cursor.pull });
                break;
            }
        }
    }
}