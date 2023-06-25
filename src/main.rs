pub mod assets_cache;
pub mod camera;
pub mod entities;
pub mod external_system;
pub mod resources;
pub mod ui;

use assets_cache::AssetsCachePlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use entities::EntitiesPlugins;
use external_system::ExternalPlugin;
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
        .run();
}
