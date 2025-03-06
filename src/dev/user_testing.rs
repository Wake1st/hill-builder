use bevy::prelude::*;

use crate::water::ShiftWater;

pub const FILL_KEY: KeyCode = KeyCode::Tab;

pub struct UserTestingPlugin;

impl Plugin for UserTestingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WaterToggle(false));
        app.add_systems(Update, toggle_water);
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
pub fn update_water_selection<E>() -> impl Fn(Trigger<E>, EventWriter<ShiftWater>) {
    move |trigger, mut selection| {
        selection.send(ShiftWater {
            entity: trigger.entity(),
            upward: true,
        });
    }
}
