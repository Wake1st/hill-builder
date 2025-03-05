use bevy::prelude::*;

use crate::{block::Block, draining::CheckDrainable, shifting::SHIFT_AMOUNT};

const FILL_KEY: KeyCode = KeyCode::Tab;

pub const WATER_MESH_SCALE: f32 = 0.98;
pub const WATER_COLOR: Color = Color::srgb(0.0, 0.2, 0.9);

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ToggleWater(false));
        app.add_event::<FillWater>().add_event::<CheckWater>();
        app.add_systems(Update, (toggle_water, check_water, fill_water));
    }
}

#[derive(Resource)]
pub struct ToggleWater(pub bool);

fn toggle_water(keys: Res<ButtonInput<KeyCode>>, mut toggle: ResMut<ToggleWater>) {
    if keys.just_pressed(FILL_KEY) {
        toggle.0 = !toggle.0;
    }
}

#[derive(Component, Debug, Default)]
pub struct Water {
    pub amount: f32,
    pub rate: f32,
}

#[derive(Event)]
pub struct CheckWater {
    pub block: Entity,
}

fn check_water(
    mut event: EventReader<CheckWater>,
    blocks: Query<&Children, With<Block>>,
    mut check_drainable: EventWriter<CheckDrainable>,
) {
    for check in event.read() {
        let Ok(children) = blocks.get(check.block) else {
            continue;
        };
        for &child in children.iter() {
            check_drainable.send(CheckDrainable {
                block: check.block,
                water: child,
            });

            break;
        }
    }
}

#[derive(Event)]
pub struct FillWater {
    pub block: Entity,
}

fn fill_water(
    mut event: EventReader<FillWater>,
    blocks: Query<&Children, With<Block>>,
    mut waters: Query<(&mut Water, &mut Transform)>,
    mut check_drainable: EventWriter<CheckDrainable>,
) {
    for fill in event.read() {
        let Ok(children) = blocks.get(fill.block) else {
            continue;
        };
        for &child in children.iter() {
            let Ok((mut water, mut transform)) = waters.get_mut(child) else {
                continue;
            };

            water.amount += SHIFT_AMOUNT;
            transform.translation.y += SHIFT_AMOUNT;

            check_drainable.send(CheckDrainable {
                block: fill.block,
                water: child,
            });

            break;
        }
    }
}
