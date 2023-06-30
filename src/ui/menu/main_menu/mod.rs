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
        app.add_system(render_menu.in_schedule(OnEnter(ApplicationState::MainMenu)))
            .add_system(destroy_menu.in_schedule(OnExit(ApplicationState::MainMenu)))
            .add_systems(
                (
                    play_button_interaction,
                    exit_button_interaction,
                )
                    .in_set(OnUpdate(ApplicationState::MainMenu)),
            );
    }
}

pub fn render_menu(commands: Commands, asset_server: Res<AssetServer>) {
    add_menu_music();
    render_menu_background();
    render_menu_ui(commands, asset_server);
}

pub fn destroy_menu(commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    remove_menu_music();
    destroy_menu_background();
    destroy_menu_ui(commands, main_menu_query);
}
