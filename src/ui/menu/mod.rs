pub mod main_menu;
pub mod pause_menu;
pub mod systems;

use bevy::prelude::{App, Plugin};
use main_menu::MainMenuPlugin;
use pause_menu::PauseMenuPlugin;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, _app: &mut App) {
        _app
            .add_plugin(MainMenuPlugin)
            .add_plugin(PauseMenuPlugin);
    }
}
