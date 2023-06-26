
use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct AsteroidProperties {
    pub position: Vec3,
    pub radius: f32,
    pub mass: f32,
}
