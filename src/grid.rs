use bevy::prelude::*;

use crate::neighborhood::Neighborhood;

pub const CELL_HEIGHT: f32 = 0.5;

#[derive(Component, Debug, Default)]
pub struct GridCell {
    pub row: i32,
    pub col: i32,
    pub layer: f32,
}

impl PartialEq for GridCell {
    fn eq(&self, rhs: &GridCell) -> bool {
        self.row == rhs.row && self.col == rhs.col
    }
}

trait GridBuilder {
    fn from_grid_coordinates(coordinations: IVec3, offset: f32) -> Self;
}

impl GridBuilder for GridCell {
    fn from_grid_coordinates(coordinates: IVec3, _offset: f32) -> Self {
        Self {
            row: coordinates.x,
            col: coordinates.y,
            layer: coordinates.z as f32 * CELL_HEIGHT,
        }
    }
}

impl GridBuilder for Transform {
    fn from_grid_coordinates(coordinates: IVec3, offset: f32) -> Self {
        Transform::from_xyz(
            coordinates.x as f32 - offset,
            coordinates.z as f32 * CELL_HEIGHT,
            coordinates.y as f32 - offset,
        )
    }
}

#[derive(Bundle)]
pub struct GridCellBundle {
    cell: GridCell,
    transform: Transform,
    neighborhood: Neighborhood,
}

impl GridCellBundle {
    pub fn new(grid_offset: f32, grid_coordinates: IVec3) -> Self {
        Self {
            transform: Transform::from_grid_coordinates(grid_coordinates, grid_offset),
            cell: GridCell::from_grid_coordinates(grid_coordinates, grid_offset),
            neighborhood: Neighborhood::default(),
        }
    }
}
