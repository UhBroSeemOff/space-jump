pub mod asteroid;

use asteroid::AsteroidPlugin;
use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

pub struct EntitiesPlugins;

impl PluginGroup for EntitiesPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(AsteroidPlugin)
    }
}
