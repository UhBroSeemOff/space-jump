use bevy::prelude::Vec2;

pub struct AsteroidProperties {
    pub position: Vec2,
    pub radius: f32,
    pub mass: f32,
}
pub struct SpawnAsteroidEvent {
    pub properties: AsteroidProperties,
}
