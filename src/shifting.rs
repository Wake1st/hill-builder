use bevy::prelude::*;

use crate::block::Block;

const SHIFT_RATE: f32 = 8.4;
pub const SHIFT_AMOUNT: f32 = 0.5;

pub struct ShiftPlugin;

impl Plugin for ShiftPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShiftFinished>()
            .add_systems(Update, (shift_blocks, shift_neighbors));
    }
}

#[derive(Component)]
pub struct Shifting {
    pub up: bool,
}

#[derive(Event)]
pub struct ShiftFinished {
    pub up: bool,
    pub row: i32,
    pub col: i32,
    pub layer: f32
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
                up: shifting.up,
                row: block.row,
                col: block.col,
                layer: block.layer
            });

            //  remove the shifting component
            commands.entity(entity).remove::<Shifting>();
        }
    }
}

fn shift_neighbors(
    mut shift_finished: EventReader<ShiftFinished>,
    mut blocks: Query<(Entity, &mut Block)>,
    mut commands: Commands
) {
    for shift in shift_finished.read() {
        for (neighbor_entity, mut neighbor_block) in blocks.iter_mut() {
            let left_neighbor = shift.row - 1 == neighbor_block.row && shift.col == neighbor_block.col;
            let right_neighbor = shift.row + 1 == neighbor_block.row && shift.col == neighbor_block.col;
            let front_neighbor = shift.col - 1 == neighbor_block.col && shift.row == neighbor_block.row;
            let back_neighbor = shift.col + 1 == neighbor_block.col && shift.row == neighbor_block.row;

            if left_neighbor || right_neighbor || front_neighbor || back_neighbor {    
                let separation = shift.layer - neighbor_block.layer;

                let layer_change= if shift.up && separation > SHIFT_AMOUNT {
                    SHIFT_AMOUNT
                } else if !shift.up && separation < -SHIFT_AMOUNT {
                    -SHIFT_AMOUNT
                } else {
                    0.0
                };
                
                if layer_change != 0.0 {
                    neighbor_block.layer += layer_change;
                    commands.entity(neighbor_entity).insert(Shifting { up: shift.up });
                }
            } 
        }
    }
}