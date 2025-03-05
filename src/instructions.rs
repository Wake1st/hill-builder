use bevy::prelude::*;

use crate::water::{toggle_water, WaterToggle, FILL_KEY};

pub struct InstructionsPlugin;

impl Plugin for InstructionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, toggle_water_display.after(toggle_water));
    }
}

#[derive(Component)]
struct WaterToggleText;

fn setup(mut commands: Commands, water_toggle: Res<WaterToggle>) {
    // Text to describe the controls.
    commands.spawn((
        Text::new("Left click a block to pull it up; right click a block to push it down."),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));

    // Text to describe the water toggle
    commands.spawn((
        Text::new(format!("Water toggled: {:?}", water_toggle.0)),
        WaterToggleText,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(36.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn toggle_water_display(
    keys: Res<ButtonInput<KeyCode>>,
    toggle: ResMut<WaterToggle>,
    mut query: Query<&mut TextSpan, With<WaterToggleText>>,
) {
    if keys.just_pressed(FILL_KEY) {
        if let Ok(mut text) = query.get_single_mut() {
            **text = format!("Water toggled: {:?}", toggle.0);
        };
    }
}
