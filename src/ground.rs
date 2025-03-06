use bevy::prelude::*;

use crate::{grid::GridCell, water::{SpawnWater, Water}};

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
    pub water_level: f32
}

fn ground_check(
    mut ground_check: EventReader<CheckGroundForDraining>,
    grounds: Query<(Entity, &GridCell, &GlobalTransform), (With<Ground>, Without<Water>)>,
    mut spawn_water: EventWriter<SpawnWater>
) {
    for check in ground_check.read() {
        //  find the matching ground cell
        for (entity, cell, transform) in grounds.iter() {
            if check.row == cell.row && check.col == cell.col {
                //  only add water if the ground is lower
                let difference = transform.translation().y - check.water_level;
                if difference < 0.0 {
                    spawn_water.send(SpawnWater {
                        ground: entity
                    });
                }
            }
        }
    }

}