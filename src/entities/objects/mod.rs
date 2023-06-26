pub mod asteroid;

use bevy::prelude::*;

use asteroid::AsteroidPlugin;

pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_plugin(AsteroidPlugin);
    }
}
