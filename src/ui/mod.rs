pub mod constants;
pub mod level_pick;
pub mod menu;
pub mod systems;

use bevy::prelude::{App, Plugin};
use level_pick::LevelPickPlugin;
use menu::MenuPlugin;

use crate::resources::{PauseMenuState, MainMenuState};

pub struct UIPlugin;

// TODO: использовать PluginGroup

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PauseMenuState>()
            .add_state::<MainMenuState>()
            .add_plugin(MenuPlugin)
            .add_plugin(LevelPickPlugin);
    }
}
