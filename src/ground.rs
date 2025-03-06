use bevy::prelude::*;

use crate::{dev::user_testing::CreateWater, grid::GridCell, water::Water};

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CheckGroundForDraining>();
        app.add_systems(Update, ground_check);
    }
}

#[derive(Component)]
pub struct Ground;

#[derive(Event)]
pub struct CheckGroundForDraining {
    pub row: i32,
    pub col: i32,
    pub water_level: f32,
}

fn ground_check(
    mut ground_check: EventReader<CheckGroundForDraining>,
    grounds: Query<(Entity, &GridCell, &GlobalTransform), (With<Ground>, Without<Water>)>,
    mut spawn_water: EventWriter<CreateWater>,
) {
    for check in ground_check.read() {
        // info!("checking ground: ({:?}, {:?})", check.row, check.col);
        //  find the matching ground cell
        for (entity, cell, transform) in grounds.iter() {
            if check.row == cell.row && check.col == cell.col {
                //  only add water if the ground is lower
                let difference = transform.translation().y - check.water_level;
                // info!("ground check diff: {:?}", difference);
                if difference < 0.0 {
                    // info!("spawning water");
                    spawn_water.send(CreateWater { ground: entity });
                }
            }
        }
    }
}
