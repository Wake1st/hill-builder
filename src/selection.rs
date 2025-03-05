use bevy::prelude::*;

use crate::water::{CheckWater, WaterToggle};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshPickingPlugin)
            .add_event::<GroundSelected>()
            .add_event::<WaterSelected>();
    }
}

#[derive(Event, Debug)]
pub struct GroundSelected {
    pub entity: Entity,
}

#[derive(Event, Debug)]
pub struct WaterSelected {
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
) -> impl Fn(Trigger<E>, Res<WaterToggle>, EventWriter<GroundSelected>, EventWriter<CheckWater>) {
    move |trigger, toggle, mut selection, mut fill| {
        if toggle.0 {
            fill.send(CheckWater {
                cell: trigger.entity(),
            });
        } else {
            selection.send(GroundSelected {
                entity: trigger.entity(),
            });
        }
    }
}

/// An observer that runs the selection event for water
pub fn update_water_selection<E>() -> impl Fn(Trigger<E>, EventWriter<WaterSelected>) {
    move |trigger, mut selection| {
        selection.send(WaterSelected {
            entity: trigger.entity(),
        });
    }
}
