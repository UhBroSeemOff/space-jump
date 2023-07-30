use bevy::{
    prelude::{
        default, info, BuildChildren, Commands, Entity, EventReader, EventWriter, Name, Query, Res,
        Transform, Vec2, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{assets_cache::resources::AssetsCache, game_play::level::components::Level};

use super::{components::Asteroid, events::AsteroidProperties, events::SpawnAsteroidEvent};

pub fn setup(mut event_writer: EventWriter<SpawnAsteroidEvent>) {
    // TODO: Remove asteroid test spawn
    // Test spawn start
    let test_positions: [Vec2; 12] = [
        Vec2::splat(250.0),
        Vec2::splat(500.0),
        Vec2::splat(-500.0),
        Vec2::splat(-250.0),
        Vec2::new(500.0, -500.0),
        Vec2::new(-500.0, 500.0),
        Vec2::new(250.0, -250.0),
        Vec2::new(-250.0, 250.0),
        Vec2::new(500.0, 0.0),
        Vec2::new(-500.0, 0.0),
        Vec2::new(0.0, 500.0),
        Vec2::new(0.0, -500.0),
    ];
    for pos in test_positions {
        let properties = AsteroidProperties {
            position: pos,
            radius: rand::thread_rng().gen_range(10.0..100.0),
        };
        event_writer.send(SpawnAsteroidEvent { properties });
    }
    // Test spawn end
}

pub fn spawn_asteroid(
    mut commands: Commands,
    asset_service: Res<AssetsCache>,
    mut event_reader: EventReader<SpawnAsteroidEvent>,
    level_query: Query<Entity, With<Level>>,
) {
    if event_reader.is_empty() {
        return;
    }

    for event in event_reader.iter() {
        let position = event.properties.position;
        let radius = event.properties.radius;

        let level = level_query.get_single();

        let asteroid = commands
            .spawn((
                Asteroid,
                RigidBody::Dynamic,
                Name::new(format!("Asteroid")),
                SpriteBundle {
                    texture: asset_service.sprites.entities.asteroid.to_owned(),
                    sprite: Sprite {
                        // Since asteroid is expected to be round,
                        // sprite's size will be somewhat equal to a asteroid's diameter
                        custom_size: Option::Some(Vec2::splat(radius * 2.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::ZERO),
                    ..default()
                },
            ))
            .insert(Collider::ball(radius))
            .insert(Restitution::coefficient(0.0))
            .insert(TransformBundle::from(Transform::from_translation(
                // 'position' is a Vec2, but it takes a Vec3 to spawn, so an absent Z-axis value should be added
                position.extend(0.0),
            )))
            .insert(GravityScale(0.0))
            .insert(Sleeping::disabled())
            .id();

        match level {
            Ok(entity) => {
                commands.entity(entity).add_child(asteroid);
            }
            Err(_) => {
                info!("Level not found");
            }
        };
    }
}
