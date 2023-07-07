pub mod assets_cache;
pub mod camera;
pub mod entities;
pub mod external_system;
pub mod game_play;
pub mod input;
pub mod resources;
pub mod ui;

use bevy::prelude::*;
use assets_cache::AssetsCachePlugin;
use camera::CameraPlugin;
use entities::EntitiesPlugins;
use external_system::ExternalPlugin;
use game_play::GamePlayPlugins;
use input::InputPlugin;
use resources::{ApplicationState, GameState};
use ui::UIPlugin;

fn main() {
    App::new()
        .add_state::<ApplicationState>()
        .add_state::<GameState>()
        .add_plugin(ExternalPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AssetsCachePlugin)
        .add_plugins(EntitiesPlugins)
        .add_plugins(GamePlayPlugins)
        .add_plugin(InputPlugin)
        .run();
}
