pub mod ui;
pub mod resources;

use resources::*;
use ui::UIPlugin;

use bevy::{prelude::*, window::*};

const RESOLUTION: Vec2 = Vec2 {
    x: 1920.0,
    y: 1080.0,
};

const TITLE: &str = "Space Jump";

fn main() {
    let default_plugin = configure_default_plugin();
    App::new()
        .add_plugins(default_plugin)
        .add_plugin(UIPlugin)
        .add_state::<ApplicationState>()
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