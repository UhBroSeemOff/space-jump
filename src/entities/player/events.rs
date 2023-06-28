use bevy::prelude::Vec3;

pub struct PlayerProperties {
    pub position: Vec3,
}

pub struct SpawnPlayerEvent {
    pub properties: PlayerProperties,
}

pub struct PlayerSpawnedEvent;
