
pub mod asteroid;

use bevy::{
    prelude::PluginGroup,
    app::PluginGroupBuilder,
};
use asteroid::AsteroidPlugin;

pub struct EntitiesPlugins;

impl PluginGroup for EntitiesPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AsteroidPlugin)
    }
}
