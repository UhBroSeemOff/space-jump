use bevy::{
    prelude::{
        info, BuildChildren, Commands, Entity, EventReader, EventWriter, Name, Query, Transform,
        Vec2, With,
    },
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::*;

use crate::camera::components::CameraTarget;
use crate::game_play::level::components::Level;

use super::{
    components::Player,
    events::{PlayerProperties, PlayerSpawnedEvent, SpawnPlayerEvent},
};

pub fn setup(mut event_writer: EventWriter<SpawnPlayerEvent>) {
    // TODO: Remove player test spawn
    // Test spawn start
    let properties: PlayerProperties = PlayerProperties {
        position: Vec2::ZERO,
    };
    event_writer.send(SpawnPlayerEvent { properties });
    // Test spawn end
}

pub fn spawn_player(
    mut commands: Commands,
    level_query: Query<Entity, With<Level>>,
    mut event_reader: EventReader<SpawnPlayerEvent>,
    mut event_writer: EventWriter<PlayerSpawnedEvent>,
) {
    if event_reader.is_empty() {
        return;
    }

    let mut position: Vec2 = Vec2::ZERO;

    for event in event_reader.iter() {
        position = event.properties.position;
    }
    let level = level_query.get_single();

    let player = commands
        .spawn((
            Player,
            CameraTarget,
            RigidBody::Dynamic,
            Name::new(format!("Player")),
        ))
        .insert(Collider::cuboid(10.0, 25.0))
        .insert(Restitution::coefficient(0.0))
        .insert(TransformBundle::from(Transform::from_translation(
            // 'position' is a Vec2, but it takes a Vec3 to spawn, so an absent Z-axis value should be added
            position.extend(0.0),
        )))
        .insert(GravityScale(0.0))
        .insert(ColliderMassProperties::Mass(10.0))
        .insert(Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        })
        .insert(Sleeping::disabled())
        .id();

    match level {
        Ok(entity) => {
            commands.entity(entity).add_child(player);
        }
        Err(_) => {
            info!("Level not found");
        }
    };

    event_writer.send(PlayerSpawnedEvent {});
}
