use bevy::prelude::*;

use crate::{dev::user_testing::WaterToggle, water::SpawnWater};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshPickingPlugin)
            .add_event::<GroundSelected>();
    }
}

#[derive(Event, Debug)]
pub struct GroundSelected {
    pub entity: Entity,
}

/// An observer that updates the entity's material to the one specified.
pub fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    // An observer closure that captures `new_material`.
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.entity()) {
            material.0 = new_material.clone();
        }
    }
}

/// An observer that runs the selection event for ground
pub fn update_ground_selection<E>(
) -> impl Fn(Trigger<E>, Res<WaterToggle>, EventWriter<GroundSelected>, EventWriter<SpawnWater>) {
    move |trigger, toggle, mut ground_selected, mut spawn_water| {
        if toggle.0 {
            spawn_water.send(SpawnWater {
                ground: trigger.entity(),
            });
        } else {
            ground_selected.send(GroundSelected {
                entity: trigger.entity(),
            });
        }
    }
}
