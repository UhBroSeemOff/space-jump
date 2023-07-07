use bevy::prelude::{Plugin, OnEnter, IntoSystemAppConfig};

pub(crate) mod components;
pub(crate) mod events;
mod systems;

use crate::resources::ApplicationState;

use self::events::{PlayerSpawnedEvent, SpawnPlayerEvent};
use self::systems::{setup, spawn_player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(setup.in_schedule(OnEnter(ApplicationState::Game)))
            .add_system(spawn_player.in_schedule(OnEnter(ApplicationState::Game)))
            .add_event::<SpawnPlayerEvent>()
            .add_event::<PlayerSpawnedEvent>();
    }
}
