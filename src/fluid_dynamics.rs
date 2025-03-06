use bevy::prelude::*;

use crate::{grid::GridCell, ground::{CheckGroundForDraining, Ground}, neighborhood::Neighborhood, water::Water};

const WATER_SPEED: f32 = 4.0;

pub struct FluidDynamicsPlugin;

impl Plugin for FluidDynamicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (check_neighbors, set_drain_rate, move_water),
        );
    }
}

fn check_neighbors(
    mut waters: Query<(Entity, &GlobalTransform, &GridCell)>,
    neighborhoods: Query<&Neighborhood, (With<Water>, Without<Ground>)>,
    mut ground_check: EventWriter<CheckGroundForDraining>,
) {
    for (water_entity, water_transform, cell) in waters.iter_mut() {
        let water_level = water_transform.translation().y;

        let Ok(neighborhood) = neighborhoods.get(water_entity) else {
            continue;
        };

        for &neighbor_entity in neighborhood.get_neighbors().iter() {
            //  if a water neighbor doesn't exist, we must create one
            if neighbor_entity == Entity::PLACEHOLDER {
                ground_check.send(CheckGroundForDraining {
                    row: cell.row,
                    col: cell.col,
                    water_level,
                });
            }
        }
    }
}

fn set_drain_rate(
    mut waters: Query<(Entity, &GlobalTransform, &mut Water)>,
    neighborhoods: Query<&Neighborhood, (With<Water>, Without<Ground>)>,
    neighbors: Query<&GlobalTransform, (With<Water>, Without<Ground>)>,
) {
    for (water_entity, water_transform, mut water) in waters.iter_mut() {
        let water_level = water_transform.translation().y;
        let mut drain_rate: f32 = 0.0;

        let Ok(neighborhood) = neighborhoods.get(water_entity) else {
            continue;
        };

        for &neighbor_entity in neighborhood.get_neighbors().iter() {
            let Ok(neighbor_transform) = neighbors.get(neighbor_entity) else {
                continue;
            };

            let difference = neighbor_transform.translation().y - water_level;
            drain_rate += difference;
        }

        if drain_rate != 0.0 {
            // info!("{:?} rate: {:?}", parent, drain_rate);
            water.rate = drain_rate;
        }
    }
}

fn move_water(mut waters: Query<(&mut Water, &mut Transform)>, time: Res<Time>) {
    let delta_time = time.delta_secs();

    for (mut water, mut transform) in waters.iter_mut() {
        let drain_amount = water.rate * WATER_SPEED * delta_time;
        water.amount += drain_amount;
        transform.translation.y += drain_amount;
    }
}
