use bevy::prelude::Plugin;

use self::trigger_zone::TriggerZonePlugin;

pub mod trigger_zone;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(TriggerZonePlugin);
    }
}
