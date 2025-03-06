use bevy::prelude::*;

use crate::{
    grid::GridCell,
    ground::{CheckGroundForDraining, Ground},
    neighborhood::Neighborhood,
    water::Water,
};

const WATER_SPEED: f32 = 0.2;

pub struct FluidDynamicsPlugin;

impl Plugin for FluidDynamicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_neighbors, set_drain_rate, move_water));
    }
}

fn check_neighbors(
    mut waters: Query<(Entity, &GlobalTransform, &GridCell), (With<Water>, Without<Ground>)>,
    neighborhoods: Query<&Neighborhood, (With<Water>, Without<Ground>)>,
    mut ground_check: EventWriter<CheckGroundForDraining>,
) {
    for (water_entity, water_transform, cell) in waters.iter_mut() {
        // info!("checking neighbors of: {:?}", water_entity);
        let water_level = water_transform.translation().y;

        let Ok(neighborhood) = neighborhoods.get(water_entity) else {
            continue;
        };

        //  if a water neighbor doesn't exist, we must check if one needs to exist
        if neighborhood.left_neighbor == Entity::PLACEHOLDER {
            ground_check.send(CheckGroundForDraining {
                row: cell.row - 1,
                col: cell.col,
                water_level,
            });
        }
        if neighborhood.right_neighbor == Entity::PLACEHOLDER {
            ground_check.send(CheckGroundForDraining {
                row: cell.row + 1,
                col: cell.col,
                water_level,
            });
        }
        if neighborhood.front_neighbor == Entity::PLACEHOLDER {
            ground_check.send(CheckGroundForDraining {
                row: cell.row,
                col: cell.col - 1,
                water_level,
            });
        }
        if neighborhood.back_neighbor == Entity::PLACEHOLDER {
            ground_check.send(CheckGroundForDraining {
                row: cell.row,
                col: cell.col + 1,
                water_level,
            });
        }
    }
}

fn set_drain_rate(
    mut waters: Query<(Entity, &GlobalTransform, &mut Water)>,
    neighborhoods: Query<&Neighborhood, (With<Water>, Without<Ground>)>,
    neighbors: Query<&GlobalTransform, (With<Water>, Without<Ground>)>,
) {
    for (water_entity, water_transform, mut water) in waters.iter_mut() {
        // info!("setting drain rate of: {:?}", water_entity);
        let water_level = water_transform.translation().y;
        let mut drain_rate: f32 = 0.0;

        let Ok(neighborhood) = neighborhoods.get(water_entity) else {
            continue;
        };

        for &neighbor_entity in neighborhood.get_neighbors().iter() {
            // info!("getting neighbor transform: {:?}", neighbor_entity);
            let Ok(neighbor_transform) = neighbors.get(neighbor_entity) else {
                continue;
            };

            let difference = neighbor_transform.translation().y - water_level;
            drain_rate += difference;
        }

        if drain_rate != 0.0 {
            // info!("setting drain rate: {:?}", drain_rate);
            water.rate = drain_rate;
        }
    }
}

fn move_water(mut waters: Query<(&mut Water, &mut Transform)>, time: Res<Time>) {
    let delta_time = time.delta_secs();

    for (mut water, mut transform) in waters.iter_mut() {
        let drain_amount = water.rate * WATER_SPEED * delta_time;
        // info!("draining amount: {:?}", drain_amount);
        water.amount += drain_amount;
        transform.translation.y += drain_amount;
    }
}
