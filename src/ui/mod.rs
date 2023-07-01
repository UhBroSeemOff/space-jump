pub mod level_pick;
pub mod menu;
pub mod systems;
pub mod constants;

use bevy::prelude::{App, Plugin};
use level_pick::LevelPickPlugin;
use menu::MenuPlugin;

pub struct UIPlugin;

// TODO: использовать PluginGroup

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MenuPlugin).add_plugin(LevelPickPlugin);
    }
}
