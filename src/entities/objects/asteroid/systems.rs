
use bevy::{
    prelude::{
        Commands,
        Transform,
        EventReader,
        Vec3,
        Vec2,
        EventWriter,
    },
    transform::TransformBundle,
};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use super::{components::AsteroidProperties, events::SpawnAsteroidEvent};

pub fn setup(
    mut event_writer: EventWriter<SpawnAsteroidEvent>,
) {

    // Test spawn
    let properties: AsteroidProperties = 
    AsteroidProperties {
        position: Vec3::new(
            rand::thread_rng().gen_range(-100.0 .. 100.0),
            rand::thread_rng().gen_range(-100.0 .. 100.0),
            rand::thread_rng().gen_range(-100.0 .. 100.0)),
        radius: rand::thread_rng().gen_range(10.0 .. 100.0),
        mass: rand::thread_rng().gen_range(1.0 .. 100.0),
    };
    event_writer.send(SpawnAsteroidEvent{properties});

}

pub fn spawn_asteroid(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnAsteroidEvent>,
) {

    if event_reader.is_empty() {
        return;
    }

    let mut position: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let mut radius: f32 = 10.0;
    let mut mass: f32 = 10.0;

    for event in event_reader.iter() {
        position = event.properties.position;
        radius = event.properties.radius;
        mass = event.properties.mass;
    }

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(radius))
        .insert(Restitution::coefficient(0.0))
        .insert(TransformBundle::from(Transform::from_translation(position)))
        .insert(GravityScale(0.0))
        .insert(ColliderMassProperties::Mass(mass))
        .insert(Velocity{
            linvel:
                Vec2::new(
                    rand::thread_rng().gen_range(0.0 .. 10.0),
                    rand::thread_rng().gen_range(0.0 .. 10.0)
                ),
            angvel: rand::thread_rng().gen_range(0.0 .. 1.0)
        })
        .insert(Sleeping::disabled())
        ;

}
