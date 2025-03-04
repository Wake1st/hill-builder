use bevy::prelude::*;

use crate::{block::{Block, Neighborhood}, selection::BlockSelected};

const SHIFT_RATE: f32 = 8.4;
pub const SHIFT_AMOUNT: f32 = 0.5;

pub struct ShiftPlugin;

impl Plugin for ShiftPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShiftFinished>()
            .add_systems(Update, (try_shift_selected_block, shift_blocks, shift_neighbors));
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

/// A function that shifts a selected block
fn try_shift_selected_block(
    mut selection: EventReader<BlockSelected>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut blocks: Query<&mut Block>,
    mut commands: Commands,
) {
    for event in selection.read() {
        let left_selected = buttons.pressed(MouseButton::Left);
        let right_selected = buttons.pressed(MouseButton::Right);

        if let Ok(mut block) = blocks.get_mut(event.entity) {
            block.layer += if left_selected {
                SHIFT_AMOUNT
            } else if right_selected {
                -SHIFT_AMOUNT
            } else {
                0.0
            };

            commands
                .entity(event.entity)
                .insert(Shifting { up: left_selected });
        }
    }
}

fn shift_blocks(
    time: Res<Time>,
    mut shifters: Query<(Entity, &Block, &mut Transform, &Shifting)>,
    mut shift_finished: EventWriter<ShiftFinished>,
    mut commands: Commands,
) {
    let delta = SHIFT_RATE * time.delta_secs();
    for (entity, block, mut transform, shifting) in shifters.iter_mut() {
        if transform.translation.y != block.layer {
            //  calculate shift and check finish
            if transform.translation.y < block.layer {
                transform.translation.y += delta;

                if transform.translation.y > block.layer {
                    transform.translation.y = block.layer;
                }
            } else if transform.translation.y > block.layer {
                transform.translation.y -= delta;

                if transform.translation.y < block.layer {
                    transform.translation.y = block.layer;
                }
            }
        }

        if transform.translation.y == block.layer {
            //  send event to check neighbors
            shift_finished.send(ShiftFinished {
                entity,
                up: shifting.up,
                layer: block.layer,
            });

            //  remove the shifting component
            commands.entity(entity).remove::<Shifting>();
        }
    }
}

fn shift_neighbors(
    mut shift_finished: EventReader<ShiftFinished>,
    blocks: Query<&Neighborhood>,
    mut neighbors: Query<&mut Block>,
    mut commands: Commands,
) {
    for shift in shift_finished.read() {
        let Ok(neighborhood) = blocks.get(shift.entity) else {
            continue;
        };

        //  shift the 4 neighbors, if necessary
        for neighbor_entity in neighborhood.get_neighbors().iter() {
            let Ok(mut neighbor_block) = neighbors.get_mut(*neighbor_entity) else {
                continue;
            };

            //  calculate if there is a layer change
            let separation = shift.layer - neighbor_block.layer;
            let layer_change = if shift.up && separation > SHIFT_AMOUNT {
                SHIFT_AMOUNT
            } else if !shift.up && separation < -SHIFT_AMOUNT {
                -SHIFT_AMOUNT
            } else {
                0.0
            };

            //  ensure the block only shifts when required
            if layer_change != 0.0 {
                neighbor_block.layer += layer_change;
                commands
                    .entity(*neighbor_entity)
                    .insert(Shifting { up: shift.up });
            }
        }
    }
}
