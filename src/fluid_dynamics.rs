use bevy::prelude::*;

use crate::{ground::Ground, neighborhood::Neighborhood, water::Water};

const WATER_SPEED: f32 = 0.02;
const LEVEL_CUTOFF: f32 = 0.05;

pub struct FluidDynamicsPlugin;

impl Plugin for FluidDynamicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddDrainingToEmptyWater>();
        app.add_systems(
            Update,
            (
                attach_draining,
                remove_draining,
                add_draining,
                set_drain_rate,
                drain_water,
            ),
        );
    }
}

#[derive(Component, Debug, Default)]
pub struct Draining {
    pub rate: f32,
}

fn attach_draining(waters: Query<(Entity, &Water)>, mut commands: Commands) {
    for (entity, water) in waters.iter() {
        if water.amount > 0.0 {
            match commands.get_entity(entity) {
                Some(mut cms) => {
                    cms.insert_if_new(Draining::default());
                }
                None => (),
            };
        }
    }
}

fn remove_draining(
    mut waters: Query<(Entity, &mut Water), With<Draining>>,
    mut commands: Commands,
) {
    for (entity, mut water) in waters.iter_mut() {
        if water.amount < 0.0 {
            // info!("removing for: {:?}", entity);
            water.amount = 0.0;
            commands.entity(entity).remove::<Draining>();
        }
    }
}

#[derive(Event)]
pub struct AddDrainingToEmptyWater {
    pub water: Entity,
}

fn add_draining(mut add_draining: EventReader<AddDrainingToEmptyWater>, mut commands: Commands) {
    for event in add_draining.read() {
        // info!("adding draining to: {:?}", event.water);
        commands
            .entity(event.water)
            .insert_if_new(Draining::default());
    }
}

fn set_drain_rate(
    mut waters: Query<(Entity, &GlobalTransform, &mut Draining)>,
    neighborhoods: Query<&Neighborhood, (With<Water>, Without<Ground>)>,
    neighbors: Query<(&GlobalTransform, &Water), (With<Water>, Without<Ground>)>,
    mut add_draining: EventWriter<AddDrainingToEmptyWater>,
) {
    for (water_entity, water_transform, mut draining) in waters.iter_mut() {
        // info!("setting drain rate of: {:?}", water_entity);
        let water_level = water_transform.translation().y;
        let mut drain_rate: f32 = 0.0;

        let Ok(neighborhood) = neighborhoods.get(water_entity) else {
            continue;
        };

        for &neighbor_entity in neighborhood.get_neighbors().iter() {
            let Ok((neighbor_transform, neighbor_water)) = neighbors.get(neighbor_entity) else {
                continue;
            };

            let difference = neighbor_transform.translation().y - water_level;
            //  no need to change when differences are so low
            if difference.abs() < LEVEL_CUTOFF {
                continue;
            }

            if neighbor_water.amount > 0.0 && difference != 0.0 {
                //  only drain when the neighbor has water
                drain_rate += difference;
            } else if difference < 0.0 && neighbor_water.amount <= 0.0 {
                //  let the neighbor drain if it needs water
                add_draining.send(AddDrainingToEmptyWater {
                    water: neighbor_entity,
                });
            }
        }

        draining.rate = match drain_rate {
            i if i == 0. => 0.0,
            i if i < 0. => -1.0,
            i if i > 0. => 1.0,
            _ => 0.0,
        };
    }
}

fn drain_water(mut waters: Query<(&mut Water, &mut Transform, &Draining)>) {
    for (mut water, mut transform, draining) in waters.iter_mut() {
        let drain_amount = draining.rate * WATER_SPEED;
        water.amount += drain_amount;
        transform.translation.y += drain_amount;
    }
}
