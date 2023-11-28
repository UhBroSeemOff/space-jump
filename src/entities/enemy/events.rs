use bevy::prelude::Vec2;

pub struct EnemyProperties {
    pub position: Vec2,
}

pub struct SpawnEnemyEvent {
    pub properties: EnemyProperties,
}

pub struct EnemySpawnedEvent;
