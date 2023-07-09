use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

pub mod projectile;

pub struct MechanicsPlugins;

impl PluginGroup for MechanicsPlugins {
    fn build(self) -> PluginGroupBuilder {
        todo!()
    }
}
