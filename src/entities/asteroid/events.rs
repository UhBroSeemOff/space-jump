
use bevy::prelude::Vec3;

pub struct AsteroidProperties {
    pub position: Vec3,
    pub radius: f32,
    pub mass: f32,
}
pub struct SpawnAsteroidEvent {
    pub properties: AsteroidProperties,
}
