use bevy::{
    prelude::{Plugin, PluginGroup, Vec2},
    utils::default,
    window::*,
    DefaultPlugins,
};
use bevy_editor_pls::EditorPlugin;
use bevy_rapier2d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

const RESOLUTION: Vec2 = Vec2 {
    x: 1920.0,
    y: 1080.0,
};

const TITLE: &str = "Space Jump";

pub struct ExternalPlugin;

impl Plugin for ExternalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let default_plugin = configure_default_plugin();

        app.add_plugins(default_plugin)
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(EditorPlugin::default());
    }
}

fn configure_default_plugin() -> bevy::app::PluginGroupBuilder {
    let window_plugin = initialize_window_plugin();
    DefaultPlugins.set(window_plugin)
}

fn initialize_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(initialize_primary_window()),
        ..default()
    }
}

fn initialize_primary_window() -> Window {
    Window {
        mode: WindowMode::BorderlessFullscreen,
        title: TITLE.to_owned(),
        resolution: WindowResolution::new(RESOLUTION.x, RESOLUTION.y),
        ..default()
    }
}
