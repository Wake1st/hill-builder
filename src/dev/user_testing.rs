use bevy::prelude::*;

use crate::{fluid_dynamics::AddDrainingToEmptyWater, grid::CELL_HEIGHT, pair::Pair, water::Water};

pub const FILL_KEY: KeyCode = KeyCode::Tab;

pub struct UserTestingPlugin;

impl Plugin for UserTestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ManuallyIncreaseWater>();
        app.insert_resource(WaterToggle(false));
        app.add_systems(Update, (toggle_water, create_water));
    }
}

#[derive(Resource)]
pub struct WaterToggle(pub bool);

#[derive(Event, Debug)]
pub struct WaterToggled;

pub fn toggle_water(
    keys: Res<ButtonInput<KeyCode>>,
    mut toggle: ResMut<WaterToggle>,
    mut toggled: EventWriter<WaterToggled>,
) {
    if keys.just_pressed(FILL_KEY) {
        toggle.0 = !toggle.0;
        toggled.send(WaterToggled);
    }
}

/// An observer that runs the selection event for water
pub fn update_water_selection<E>() -> impl Fn(Trigger<E>, EventWriter<ManuallyIncreaseWater>) {
    move |trigger, mut increase| {
        increase.send(ManuallyIncreaseWater {
            ground: trigger.entity(),
        });
    }
}

#[derive(Event)]
pub struct ManuallyIncreaseWater {
    pub ground: Entity,
}

fn create_water(
    mut event: EventReader<ManuallyIncreaseWater>,
    pairs: Query<&Pair>,
    mut waters: Query<(Entity, &mut Water, &mut Transform)>,
    mut add_draining: EventWriter<AddDrainingToEmptyWater>,
) {
    for check in event.read() {
        //  find current water, if exists
        for pair in pairs.iter() {
            if pair.ground == check.ground {
                //  increase the water amount and attach drainable to it
                if let Ok((water_entity, mut water, mut transform)) = waters.get_mut(pair.water) {
                    water.amount += CELL_HEIGHT;
                    transform.translation.y += CELL_HEIGHT;

                    add_draining.send(AddDrainingToEmptyWater {
                        water: water_entity,
                    });
                };

                break;
            }
        }
    }
}
