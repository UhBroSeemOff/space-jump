use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

pub mod asteroid;
pub mod player;
pub mod enemy;

use asteroid::AsteroidPlugin;
use player::PlayerPlugin;
use enemy::EnemyPlugin;

pub struct EntitiesPlugins;

impl PluginGroup for EntitiesPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AsteroidPlugin)
            .add(PlayerPlugin)
            .add(EnemyPlugin)
    }
}
