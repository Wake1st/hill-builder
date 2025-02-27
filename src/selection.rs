use bevy::prelude::*;

use crate::{
    block::Block,
    shifting::{Shifting, SHIFT_AMOUNT},
};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshPickingPlugin)
            .add_event::<BlockSelected>()
            .add_systems(Update, try_shift_selected_block);
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
pub fn update_block_selection<E>() -> impl Fn(Trigger<E>, EventWriter<BlockSelected>) {
    move |trigger, mut selection| {
        selection.send(BlockSelected {
            entity: trigger.entity(),
        });
    }
}

/// A function that shifts a selected block
fn try_shift_selected_block(
    mut selection: EventReader<BlockSelected>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut blocks: Query<&mut Block>,
    mut commands: Commands,
) {
    for event in selection.read() {
        let left_selected = buttons.pressed(MouseButton::Left);
        let right_selected = buttons.pressed(MouseButton::Right);

        if let Ok(mut block) = blocks.get_mut(event.entity) {
            block.layer += if left_selected {
                SHIFT_AMOUNT
            } else if right_selected {
                -SHIFT_AMOUNT
            } else {
                0.0
            };

            commands
                .entity(event.entity)
                .insert(Shifting { up: left_selected });
        }
    }
}
