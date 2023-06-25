use bevy::prelude::Plugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

pub struct ExternalPlugin;

impl Plugin for ExternalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(WorldInspectorPlugin::default());
    }
}
