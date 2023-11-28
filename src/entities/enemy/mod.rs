use bevy::prelude::{Plugin, OnEnter, IntoSystemAppConfig};

pub(crate) mod components;
pub(crate) mod events;
mod systems;

use crate::resources::ApplicationState;

use self::events::{EnemySpawnedEvent, SpawnEnemyEvent};
use self::systems::spawn_enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(spawn_enemy.in_schedule(OnEnter(ApplicationState::Game)))
            .add_event::<SpawnEnemyEvent>()
            .add_event::<EnemySpawnedEvent>();
    }
}
