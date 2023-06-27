pub mod resources;
pub mod ui;
pub mod assets_cache;

use resources::*;
use ui::UIPlugin;
use assets_cache::AssetsCachePlugin;

use bevy::{prelude::*, window::*};
use camera::CameraPlugin;
use external_system::ExternalPlugin;
use entities::EntitiesPlugins;

pub mod camera;
pub mod external_system;
pub mod entities;

const RESOLUTION: Vec2 = Vec2 {
    x: 1920.0,
    y: 1080.0,
};

const TITLE: &str = "Space Jump";

fn main() {
    let default_plugin = configure_default_plugin();
    App::new()
        .add_state::<ApplicationState>()
        .add_plugins(default_plugin)
        .add_plugin(ExternalPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AssetsCachePlugin)
        .add_plugins(EntitiesPlugins)
        .run();
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
