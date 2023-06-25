use bevy::prelude::Plugin;

use self::systems::setup_camera;

mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup_camera);
    }
}
