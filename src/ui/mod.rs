pub mod menu;

use menu::MenuPlugin;
use bevy::prelude::{App, Plugin};

pub struct UIPlugin;

// TODO: использовать PluginGroup

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MenuPlugin);
    }
}

