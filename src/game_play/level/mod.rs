pub mod components;
pub mod events;
mod systems;

use bevy::prelude::{IntoSystemConfig, Plugin};

use self::{
    components::Level,
    events::{LoadLevelEvent, UnloadLevelEvent},
    systems::*,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<LoadLevelEvent>()
            .register_type::<Level>()
            .add_event::<UnloadLevelEvent>()
            .add_system(load_level)
            .add_system(unload_level.before(load_level))
            .add_startup_system(load_test_scene);
    }
}
