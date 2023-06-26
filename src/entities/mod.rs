pub mod objects;

use bevy::prelude::*;

use objects::ObjectsPlugin;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_plugin(ObjectsPlugin);
    }
}
