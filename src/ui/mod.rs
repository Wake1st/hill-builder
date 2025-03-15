use bevy::prelude::*;
use bevy_lunex::{UiLunexDebugPlugin, UiLunexPlugins};

mod main_menu;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            main_menu::MainMenuPlugin,
            UiLunexPlugins, 
            UiLunexDebugPlugin::<0, 0>
        ));
    }
}

