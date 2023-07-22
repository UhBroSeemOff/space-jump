pub struct MainMenuPlugin;

use crate::ApplicationState;
use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_menu_background.in_schedule(OnEnter(ApplicationState::MainMenu)))
            .add_system(destroy_menu_background.in_schedule(OnEnter(ApplicationState::Game)))
            .add_system(render_menu.in_schedule(OnEnter(ApplicationState::MainMenu)))
            .add_system(destroy_menu.in_schedule(OnExit(ApplicationState::MainMenu)))
            .add_systems(
                (play_button_interaction, exit_button_interaction)
                    .in_set(OnUpdate(ApplicationState::MainMenu)),
            );
    }
}
