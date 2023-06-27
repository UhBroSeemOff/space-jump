pub mod main_menu;
pub mod systems;

use bevy::prelude::{App, Plugin};
use main_menu::MainMenuPlugin;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MainMenuPlugin);
    }
}
