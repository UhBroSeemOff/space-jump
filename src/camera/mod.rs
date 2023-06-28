use bevy::prelude::Plugin;

use self::systems::{follow_target, setup_camera};

pub(crate) mod components;
mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup_camera)
            .add_system(follow_target);
    }
}
