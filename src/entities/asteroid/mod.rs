use bevy::prelude::{IntoSystemAppConfig, OnEnter, Plugin};

mod components;
mod events;
mod systems;

use crate::resources::ApplicationState;

use self::events::SpawnAsteroidEvent;
use self::systems::{setup, spawn_asteroid};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(setup.in_schedule(OnEnter(ApplicationState::Game)))
            .add_system(spawn_asteroid)
            .add_event::<SpawnAsteroidEvent>();
    }
}
