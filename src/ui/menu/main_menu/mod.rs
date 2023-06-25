pub struct MainMenuPlugin;

use crate::ApplicationState;
use bevy::prelude::{App, IntoSystemAppConfig, OnEnter, OnExit, Plugin};

pub mod components;
pub mod constants;
pub mod systems;

use components::*;
use systems::*;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(initialize_menu).add_system(destroy_menu);
        // app.add_system(initialize_menu.in_schedule(OnEnter(ApplicationState::MainMenu)))
        //     .add_system(destroy_menu.in_schedule(OnExit(ApplicationState::MainMenu)));
    }
}

pub fn initialize_menu() {
    render_menu_background();
    add_menu_music();
    render_menu_ui();
}

pub fn destroy_menu() {
    destroy_menu_background();
    remove_menu_music();
    destroy_menu_ui();
}
