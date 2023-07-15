use bevy::prelude::Vec2;

pub struct PlayerProperties {
    pub position: Vec2,
}

pub struct SpawnPlayerEvent {
    pub properties: PlayerProperties,
}

pub struct PlayerSpawnedEvent;
