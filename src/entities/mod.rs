use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

pub mod asteroid;
pub mod player;

use asteroid::AsteroidPlugin;
use player::PlayerPlugin;

pub struct EntitiesPlugins;

impl PluginGroup for EntitiesPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AsteroidPlugin)
            .add(PlayerPlugin)
    }
}
