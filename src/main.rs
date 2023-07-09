pub mod assets_cache;
pub mod camera;
pub mod entities;
pub mod external_system;
pub mod game_play;
pub mod input;
pub mod mechanics;
pub mod resources;
pub mod ui;

use assets_cache::AssetsCachePlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use entities::EntitiesPlugins;
use external_system::ExternalPlugin;
use game_play::GamePlayPlugins;
use input::InputPlugin;
use mechanics::MechanicsPlugins;
use resources::ApplicationState;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_state::<ApplicationState>()
        .add_plugin(ExternalPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AssetsCachePlugin)
        .add_plugins(EntitiesPlugins)
        .add_plugins(GamePlayPlugins)
        .add_plugins(MechanicsPlugins)
        .add_plugin(InputPlugin)
        .run();
}
