use bevy::prelude::*;

use crate::{block::Block, draining::CheckDrainable, shifting::SHIFT_AMOUNT};

const FILL_KEY: KeyCode = KeyCode::Tab;

pub const WATER_MESH_SCALE: f32 = 0.98;
pub const WATER_COLOR: Color = Color::srgb(0.0, 0.2, 0.9);

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ToggleWater(false));
        app.add_event::<FillWater>();
        app.add_systems(Update, (toggle_water, fill_water));
    }
}

#[derive(Resource)]
pub struct ToggleWater(pub bool);

fn toggle_water(keys: Res<ButtonInput<KeyCode>>, mut toggle: ResMut<ToggleWater>) {
    if keys.just_pressed(FILL_KEY) {
        toggle.0 = !toggle.0;
    }
}

#[derive(Event)]
pub struct FillWater {
    pub entity: Entity,
}

#[derive(Component, Debug, Default)]
pub struct Water {
    pub amount: f32,
}

fn fill_water(
    mut event: EventReader<FillWater>,
    blocks: Query<&Children, With<Block>>,
    mut waters: Query<(&mut Water, &mut Transform)>,
    mut check_drainable: EventWriter<CheckDrainable>,
) {
    for fill in event.read() {
        let Ok(children) = blocks.get(fill.entity) else {
            continue;
        };
        for &child in children.iter() {
            let Ok((mut water, mut transform)) = waters.get_mut(child) else {
                continue;
            };
    
            water.amount = SHIFT_AMOUNT;
            transform.translation.y = 0.0;

            check_drainable.send(CheckDrainable {
                block: fill.entity,
                water: child,
            });

            break;
        }
    }
}
