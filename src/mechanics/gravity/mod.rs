pub mod components;
mod systems;

use bevy::prelude::Plugin;

const NUTONIAN_CONSTANT: f32 = 6.6743015 / (10_i64.pow(11) as f32);

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {}
}
