pub struct MainMenuPlugin;

use crate::ApplicationState;
use bevy::prelude::*;

pub mod components;
pub mod constants;
pub mod systems;

use components::*;
use systems::*;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(initialize_menu.in_schedule(OnEnter(ApplicationState::MainMenu)))
            .add_system(destroy_menu.in_schedule(OnExit(ApplicationState::MainMenu)));
    }
}

pub fn initialize_menu(commands: Commands, asset_server: Res<AssetServer>) {
    render_menu_background();
    add_menu_music();
    render_menu_ui(commands, asset_server);
}

pub fn destroy_menu() {
    destroy_menu_background();
    remove_menu_music();
    destroy_menu_ui();
}
