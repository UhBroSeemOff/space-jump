use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

pub mod projectile;

use projectile::ProjectilePlugin;

pub struct MechanicsPlugins;

impl PluginGroup for MechanicsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(ProjectilePlugin)
    }
}
