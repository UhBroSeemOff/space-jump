pub mod components;
mod systems;

use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin,
};
use systems::*;

use crate::resources::ApplicationState;

pub struct LevelPickPlugin;

impl Plugin for LevelPickPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_level_pick_screen.in_schedule(OnEnter(ApplicationState::LevelPick)))
            .add_system(destroy_level_pick_screen.in_schedule(OnExit(ApplicationState::LevelPick)))
            .add_systems(
                (back_button_interaction, level_button_interaction)
                    .in_set(OnUpdate(ApplicationState::LevelPick)),
            );
    }
}
