
use bevy::prelude::{Plugin, IntoSystemAppConfig, OnEnter};

mod systems;
mod components;
mod events;

use crate::resources::ApplicationState;

use self::systems::{
    setup,
    spawn_asteroid
};
use self::events::SpawnAsteroidEvent;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_system(setup.in_schedule(OnEnter(ApplicationState::Game)))
            .add_system(spawn_asteroid.in_schedule(OnEnter(ApplicationState::Game)))
            .add_event::<SpawnAsteroidEvent>()
            ;
    }
}
