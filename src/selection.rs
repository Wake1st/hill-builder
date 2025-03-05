use bevy::prelude::*;

use crate::water::{FillWater, ToggleWater};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshPickingPlugin)
            .add_event::<BlockSelected>();
    }
}

#[derive(Event, Debug)]
pub struct BlockSelected {
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

/// An observer that runs the selection event
pub fn update_block_selection<E>(
) -> impl Fn(Trigger<E>, Res<ToggleWater>, EventWriter<BlockSelected>, EventWriter<FillWater>) {
    move |trigger, toggle, mut selection, mut fill| {
        if toggle.0 {
            fill.send(FillWater {
                block: trigger.entity(),
            });
        } else {
            selection.send(BlockSelected {
                entity: trigger.entity(),
            });
        }
    }
}
