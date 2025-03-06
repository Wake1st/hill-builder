use bevy::prelude::*;

use crate::{
    water::CheckWater,
    grid::{GridCell, CELL_HEIGHT},
    ground::Ground,
    neighborhood::Neighborhood,
    selection::GroundSelected,
};

const SHIFT_RATE: f32 = 8.4;

pub struct ShiftPlugin;

impl Plugin for ShiftPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShiftFinished>().add_systems(
            Update,
            (try_shift_selected_cell, shift_cells, shift_neighbors),
        );
    }
}

#[derive(Component)]
pub struct Shifting {
    pub up: bool,
}

#[derive(Event)]
pub struct ShiftFinished {
    pub entity: Entity,
    pub up: bool,
    pub layer: f32,
}

/// A function that shifts a selected cell
fn try_shift_selected_cell(
    mut selection: EventReader<GroundSelected>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut cells: Query<&mut GridCell>,
    mut commands: Commands,
) {
    for event in selection.read() {
        let left_selected = buttons.pressed(MouseButton::Left);
        let right_selected = buttons.pressed(MouseButton::Right);

        if let Ok(mut cell) = cells.get_mut(event.entity) {
            cell.layer += if left_selected {
                CELL_HEIGHT
            } else if right_selected {
                -CELL_HEIGHT
            } else {
                0.0
            };

            commands
                .entity(event.entity)
                .insert(Shifting { up: left_selected });
        }
    }
}

fn shift_cells(
    time: Res<Time>,
    mut shifters: Query<(Entity, &GridCell, &mut Transform, &Shifting), With<Ground>>,
    mut shift_finished: EventWriter<ShiftFinished>,
    mut check_water: EventWriter<CheckWater>,
    mut commands: Commands,
) {
    let delta = SHIFT_RATE * time.delta_secs();
    for (entity, cell, mut transform, shifting) in shifters.iter_mut() {
        if transform.translation.y != cell.layer {
            //  calculate shift and check finish
            if transform.translation.y < cell.layer {
                transform.translation.y += delta;

                if transform.translation.y > cell.layer {
                    transform.translation.y = cell.layer;
                }
            } else if transform.translation.y > cell.layer {
                transform.translation.y -= delta;

                if transform.translation.y < cell.layer {
                    transform.translation.y = cell.layer;
                }
            }
        }

        if transform.translation.y == cell.layer {
            //  send event to check neighbors
            shift_finished.send(ShiftFinished {
                entity,
                up: shifting.up,
                layer: cell.layer,
            });

            //  send event to check the water level
            check_water.send(CheckWater { cell: entity, shifting_upward: shifting.up });

            //  remove the shifting component
            commands.entity(entity).remove::<Shifting>();
        }
    }
}

fn shift_neighbors(
    mut shift_finished: EventReader<ShiftFinished>,
    cells: Query<&Neighborhood, With<Ground>>,
    mut neighbors: Query<&mut GridCell>,
    mut commands: Commands,
) {
    for shift in shift_finished.read() {
        let Ok(neighborhood) = cells.get(shift.entity) else {
            continue;
        };

        //  shift the 4 neighbors, if necessary
        for neighbor_entity in neighborhood.get_neighbors().iter() {
            let Ok(mut neighbor_cell) = neighbors.get_mut(*neighbor_entity) else {
                continue;
            };

            //  calculate if there is a layer change
            let separation = shift.layer - neighbor_cell.layer;
            let layer_change = if shift.up && separation > CELL_HEIGHT {
                CELL_HEIGHT
            } else if !shift.up && separation < -CELL_HEIGHT {
                -CELL_HEIGHT
            } else {
                0.0
            };

            //  ensure the cell only shifts when required
            if layer_change != 0.0 {
                neighbor_cell.layer += layer_change;
                commands
                    .entity(*neighbor_entity)
                    .insert(Shifting { up: shift.up });
            }
        }
    }
}
