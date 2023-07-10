use bevy::prelude::Plugin;

mod components;
mod events;
mod systems;

use self::{
    events::ProjectileCollisionEvent,
    systems::{send_collision_event, setup_projectile_events},
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(setup_projectile_events)
            .add_system(send_collision_event)
            .add_event::<ProjectileCollisionEvent>();
    }
}
