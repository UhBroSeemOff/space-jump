
use bevy::prelude::{Plugin};

mod systems;
mod components;
mod events;

use self::systems::{setup, spawn_asteroid};
use self::events::SpawnAsteroidEvent;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_startup_system(setup)
            .add_system(spawn_asteroid)
            .add_event::<SpawnAsteroidEvent>();
    }
}
