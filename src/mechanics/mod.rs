use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

use self::gravity::GravityPlugin;

pub mod gravity;

pub struct MechanicsPlugins;

impl PluginGroup for MechanicsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(GravityPlugin)
    }
}
