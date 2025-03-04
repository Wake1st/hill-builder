use bevy::prelude::*;

use crate::{neighborhood::Neighborhood, shifting::SHIFT_AMOUNT};

const BLOCK_GAP: f32 = 0.1;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, _app: &mut App) {}
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
            layer: coordinates.z as f32 * SHIFT_AMOUNT,
        }
    }
}

impl GridBuilder for Transform {
    fn from_grid_coordinates(coordinates: IVec3, offset: f32) -> Self {
        Transform::from_xyz(
            (coordinates.x as f32) * (1.0 + BLOCK_GAP) - offset,
            coordinates.z as f32 * SHIFT_AMOUNT,
            (coordinates.y as f32) * (1.0 + BLOCK_GAP) - offset,
        )
    }
}

#[derive(Bundle)]
pub struct BlockBundle {
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    block: Block,
    neighborhood: Neighborhood,
}

impl BlockBundle {
    pub fn new(
        block_mesh_handle: Handle<Mesh>,
        mesh_matl: Handle<StandardMaterial>,
        block_offset: f32,
        grid_coordinates: IVec3,
    ) -> Self {
        Self {
            mesh: Mesh3d(block_mesh_handle),
            material: MeshMaterial3d(mesh_matl),
            transform: Transform::from_grid_coordinates(grid_coordinates, block_offset),
            block: Block::from_grid_coordinates(grid_coordinates, block_offset),
            neighborhood: Neighborhood {
                left_neighbor: Entity::PLACEHOLDER,
                right_neighbor: Entity::PLACEHOLDER,
                front_neighbor: Entity::PLACEHOLDER,
                back_neighbor: Entity::PLACEHOLDER,
            },
        }
    }
}
