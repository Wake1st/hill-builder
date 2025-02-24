use bevy::prelude::*;

use crate::{cursor::StoreCursor, shifting::{Shifting, SHIFT_AMOUNT}};

const BLOCK_GAP: f32 = 0.1;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, read_position);
    }
}

#[derive(Component)]
pub struct Block {
    pub row: i32,
    pub col: i32,
    pub layer: f32,
}

trait GridBuilder {
    fn from_grid_coordinates(coordinations: IVec3, offset: f32) -> Self;
}

impl GridBuilder for Block {
    fn from_grid_coordinates(coordinates: IVec3, _offset: f32) -> Self {
        Self {
            row: coordinates.x, 
            col: coordinates.y, 
            layer: coordinates.z as f32
        }
    }
}

impl GridBuilder for Transform {
    fn from_grid_coordinates(coordinates: IVec3, offset: f32) -> Self {
        Transform::from_xyz(
            (coordinates.x as f32)*(1.0 + BLOCK_GAP) - offset, 
            coordinates.z as f32,
            (coordinates.y as f32)*(1.0 + BLOCK_GAP) - offset
        )
    }
}

#[derive(Bundle)]
pub struct BlockBundle {
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    block: Block,
}

impl BlockBundle {
    pub fn new(
        block_mesh_handle: Handle<Mesh>, 
        materials: &mut Assets<StandardMaterial>, 
        color: Color, 
        block_offset: f32, 
        grid_coordinates: IVec3
    ) -> Self {
        Self {
            mesh: Mesh3d(block_mesh_handle),
            material: MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                ..default()
            })),
            transform: Transform::from_grid_coordinates(grid_coordinates, block_offset),
            block: Block::from_grid_coordinates(grid_coordinates, block_offset),
        }
    }
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