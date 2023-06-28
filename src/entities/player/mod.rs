use bevy::prelude::Plugin;

pub(crate) mod components;
pub(crate) mod events;
mod systems;

use self::events::{PlayerSpawnedEvent, SpawnPlayerEvent};
use self::systems::{setup, spawn_player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup)
            .add_system(spawn_player)
            .add_event::<SpawnPlayerEvent>()
            .add_event::<PlayerSpawnedEvent>();
    }
}
