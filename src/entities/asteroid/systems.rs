use bevy::{
    prelude::{
        info, BuildChildren, Commands, Entity, EventReader, EventWriter, Name, Query, Transform,
        Vec2, With,
    },
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::game_play::level::components::Level;

use super::{components::Asteroid, events::AsteroidProperties, events::SpawnAsteroidEvent};

pub fn setup(mut event_writer: EventWriter<SpawnAsteroidEvent>) {
    // TODO: Remove asteroid test spawn
    // Test spawn start
    let properties: AsteroidProperties = AsteroidProperties {
        position: Vec2::new(
            rand::thread_rng().gen_range(-100.0..100.0),
            rand::thread_rng().gen_range(-100.0..100.0),
        ),
        radius: rand::thread_rng().gen_range(10.0..100.0),
        mass: rand::thread_rng().gen_range(1.0..100.0),
    };
    event_writer.send(SpawnAsteroidEvent { properties });
    // Test spawn end
}

pub fn spawn_asteroid(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnAsteroidEvent>,
    level_query: Query<Entity, With<Level>>,
) {
    if event_reader.is_empty() {
        return;
    }

    let mut position: Vec2 = Vec2::ZERO;
    let mut radius: f32 = 10.0;
    let mut mass: f32 = 10.0;

    for event in event_reader.iter() {
        position = event.properties.position;
        radius = event.properties.radius;
        mass = event.properties.mass;
    }
    let level = level_query.get_single();

    let asteroid = commands
        .spawn((Asteroid, RigidBody::Dynamic, Name::new(format!("Asteroid"))))
        .insert(Collider::ball(radius))
        .insert(Restitution::coefficient(0.0))
        .insert(TransformBundle::from(Transform::from_translation(
            // 'position' is a Vec2, but it takes a Vec3 to spawn, so an absent Z-axis value should be added
            position.extend(0.0),
        )))
        .insert(GravityScale(0.0))
        .insert(ColliderMassProperties::Mass(mass))
        .insert(Velocity {
            linvel: Vec2::new(
                rand::thread_rng().gen_range(0.0..10.0),
                rand::thread_rng().gen_range(0.0..10.0),
            ),
            angvel: rand::thread_rng().gen_range(0.0..1.0),
        })
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
