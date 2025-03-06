mod console_commands;
mod instructions;
pub mod user_testing;

use bevy::prelude::*;
use console_commands::ConComPlugin;
use instructions::InstructionsPlugin;
use user_testing::UserTestingPlugin;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ConComPlugin, InstructionsPlugin, UserTestingPlugin));
    }
}
