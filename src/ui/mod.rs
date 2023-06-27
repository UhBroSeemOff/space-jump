pub mod menu;
pub mod level_pick;

use level_pick::LevelPickPlugin;
use menu::MenuPlugin;
use bevy::prelude::{App, Plugin};

pub struct UIPlugin;

// TODO: использовать PluginGroup

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MenuPlugin).add_plugin(LevelPickPlugin);
    }
}

