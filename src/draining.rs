use bevy::prelude::*;

use crate::{block::Block, neighborhood::Neighborhood, water::Water};

pub const DRAIN_LIMIT: f32 = -0.01;

pub struct DrainingPlugin;

impl Plugin for DrainingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CheckDrainable>();
        app.add_systems(Update, (check_drainable, set_drain_rate, drain, remove_draining));
    }
}

#[derive(Component, Debug, Default)]
pub struct Draining {
    pub rate: f32,
}

#[derive(Event)]
pub struct CheckDrainable {
    pub block: Entity,
    pub water: Entity,
}

fn check_drainable(
    mut event: EventReader<CheckDrainable>,
    blocks: Query<(&Block, &Neighborhood)>,
    neighbors: Query<&Block>,
    mut commands: Commands,
) {
    for check in event.read() {
        let mut draining_amount: f32 = 0.0;

        let Ok((block, neighborhood)) = blocks.get(check.block) else {
            continue;
        };

        //  add the draining value
        for neighbor_entity in neighborhood.get_neighbors().iter() {
            let Ok(neighbor_block) = neighbors.get(*neighbor_entity) else {
                continue;
            };

            //  calculate if there is a layer change
            let separation = (neighbor_block.layer - block.layer).max(0.0);
            draining_amount += separation;
        }

        //  ensure the block only shifts when required
        if draining_amount < 0.0 {
            commands.entity(check.water).insert(Draining::default());
        }
    }
}

fn set_drain_rate (
    mut drainers: Query<(&Parent, &GlobalTransform, &mut Draining)>,
    blocks: Query<&Neighborhood>,
    neighbors: Query<&Children, With<Block>>,
    waters: Query<&GlobalTransform, With<Water>>,
) {
    for (parent, water_transform, mut draining) in drainers.iter_mut() {
        let water_level = water_transform.translation().y;
        let mut drain_rate: f32 = 0.0;

        let Ok(neighborhood) = blocks.get(**parent) else {
            continue;
        };

        for &neighbor_entity in neighborhood.get_neighbors().iter() {
            let Ok(children) = neighbors.get(neighbor_entity) else {
                continue;
            };

            for &child in children.iter() {
                let Ok(neighbor_transform) = waters.get(child) else {
                    continue;
                };

                let difference = neighbor_transform.translation().y - water_level;
                drain_rate += difference;
            }
        }

        draining.rate = drain_rate;
    }
}

fn drain (
    mut waters: Query<(&mut Water, &mut Transform, &Draining)>, 
    time: Res<Time>
) {
    let delta_time = time.delta_secs();

    for (mut water, mut transform, drainging) in waters.iter_mut() {
        let drain_amount = drainging.rate * delta_time;
        water.amount -= drain_amount;
        transform.translation.y -= drain_amount;
    }
}

fn remove_draining(waters: Query<(Entity, &Water)>, mut commands: Commands) {
    for (entity, water) in waters.iter() {
        if water.amount < DRAIN_LIMIT {
            commands.entity(entity).remove::<Draining>();
        }
    }
}
