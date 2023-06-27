use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use self::{core::CorePlugin, level::LevelPlugin};

pub mod core;
pub mod level;

pub struct GamePlayPlugins;

impl PluginGroup for GamePlayPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePlugin)
            .add(LevelPlugin)
    }
}
