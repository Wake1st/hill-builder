use bevy::prelude::*;

use crate::{grid::CELL_HEIGHT, ground::Ground, pair::Pair};

pub const WATER_MESH_SCALE: f32 = 0.98;
pub const WATER_COLOR: Color = Color::srgb(0.0, 0.2, 0.9);

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TryShiftWater>().add_event::<ShiftWater>();
        app.add_systems(Update, (try_shift_water, shift_water));
    }
}

#[derive(Component, Debug, Default)]
pub struct Water {
    pub amount: f32,
}

#[derive(Event)]
pub struct TryShiftWater {
    pub ground: Entity,
    pub shifting_upward: bool,
}

fn try_shift_water(
    mut event: EventReader<TryShiftWater>,
    grounds: Query<Entity, (With<Ground>, Without<Water>)>,
    pairs: Query<&Pair>,
    mut water_selected: EventWriter<ShiftWater>,
) {
    for check in event.read() {
        //  get the ground data
        let Ok(ground_entity) = grounds.get(check.ground) else {
            continue;
        };

        //  find current water, if exists
        for pair in pairs.iter() {
            if pair.ground == ground_entity {
                water_selected.send(ShiftWater {
                    entity: pair.water,
                    upward: check.shifting_upward,
                });
                break;
            }
        }
    }
}

#[derive(Event, Debug)]
pub struct ShiftWater {
    pub entity: Entity,
    pub upward: bool,
}

fn shift_water(
    mut event: EventReader<ShiftWater>,
    mut waters: Query<&mut Transform, Without<Ground>>,
) {
    for shift in event.read() {
        let Ok(mut transform) = waters.get_mut(shift.entity) else {
            continue;
        };

        //  update found water
        let direction = if shift.upward { 1.0 } else { -1.0 };
        transform.translation.y += direction * CELL_HEIGHT;
    }
}
