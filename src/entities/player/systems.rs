use bevy::{
    math::Vec3Swizzles,
    prelude::{
        info, BuildChildren, Camera, Commands, Entity, EventReader, EventWriter, GlobalTransform,
        Name, Query, Transform, Vec2, With,
    },
    transform::TransformBundle,
    window::Window,
};
use bevy_rapier2d::prelude::*;

use crate::{
    camera::components::{CameraTarget, MainCamera},
    input::events::InputActionEvent,
};
use crate::{game_play::level::components::Level, input::InputAction};

use super::{
    components::{Player, JUMP_ACCELERATION},
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
        // Though the 'ExternalImpulse' field is not being filed here,
        // it is still needs to be 'inserted', so it can be changed later
        .insert(ExternalImpulse {
            impulse: Vec2::ZERO,
            torque_impulse: 0.0,
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

pub fn jump(
    mut player_impulses: Query<&mut ExternalImpulse, With<Player>>,
    player_transform: Query<&Transform, With<Player>>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut event_reader: EventReader<InputActionEvent>,
) {
    if event_reader.is_empty() {
        return;
    }

    let mut action: InputAction = InputAction::Default;
    for event in event_reader.iter() {
        action = event.action();
    }

    match action {
        InputAction::Jump => {
            // Cursor position by itself - is a local "in window" coordinate,
            // so it needs to be translated into world position
            let window = window_query.single();
            let (camera, camera_transform) = camera_query.single();

            if let Some(cursor_world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
            {
                if let Ok(player) = player_transform.get_single() {
                    // The resulting impulse vector - is a difference between a cursor's translated and
                    // a player's positions, then normalized and multiplied by a speed value
                    let vector = (cursor_world_position - player.translation.xy()).normalize()
                        * JUMP_ACCELERATION;

                    for mut ext_impulse in player_impulses.iter_mut() {
                        ext_impulse.impulse = vector;
                    }
                }
            }
        }
        _ => {}
    }
}
