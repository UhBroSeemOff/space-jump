pub struct PauseMenuPlugin;

use crate::resources::{GameState, ApplicationState};
use bevy::prelude::*;

pub mod components;
pub mod systems;

use components::*;
use systems::*;

impl Plugin for PauseMenuPlugin {
    fn build(&self, _app: &mut App) {
        _app.add_system(render_menu.in_schedule(OnEnter(GameState::PauseMenu)))
            .add_system(set_pause_menu_state)
            .add_system(destroy_menu.in_schedule(OnExit(GameState::Game)))
            .add_system(destroy_menu.in_schedule(OnEnter(ApplicationState::MainMenu)))
            .add_systems(
                (resume_button_interaction, settings_button_interaction, main_menu_button_interaction)
                    .in_set(OnUpdate(GameState::PauseMenu))
            )
            .add_system(resume_button_interaction.in_set(OnUpdate(GameState::Game)));
    }
}

pub fn render_menu(commands: Commands, asset_server: Res<AssetServer>) {
    render_menu_ui(commands, asset_server);
}

pub fn destroy_menu(commands: Commands, pause_menu_query: Query<Entity, With<PauseMenu>>) {
    destroy_menu_ui(commands, pause_menu_query);
}
