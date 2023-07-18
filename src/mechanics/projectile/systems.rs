use bevy::prelude::{Entity, EventReader, EventWriter, Query, With};
use bevy_rapier2d::prelude::*;

use super::{components::Projectile, events::ProjectileCollisionEvent};

pub fn setup_projectile_events(mut projectile_query: Query<&mut ActiveEvents, With<Projectile>>) {
    for mut projectile in projectile_query.iter_mut() {
        *projectile = ActiveEvents::COLLISION_EVENTS;
    }
}

pub fn send_collision_event(
    mut event_reader: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<ProjectileCollisionEvent>,
    query: Query<Entity, With<Projectile>>,
) {
    if event_reader.is_empty() {
        return;
    }

    let self_entity: Entity = query.get_single().unwrap();
    let mut event_collision_source: Entity = Entity::PLACEHOLDER;
    let mut event_collision_target: Entity = Entity::PLACEHOLDER;

    for collision_event in event_reader.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                // It is unknown which 'entity' from 'Started()' is which,
                // so here goes ugly comparison with 'self_entity'
                if *entity1 == self_entity {
                    event_collision_source = *entity1;
                    event_collision_target = *entity2;
                } else if *entity2 == self_entity {
                    event_collision_source = *entity2;
                    event_collision_target = *entity1;
                }
            }
            CollisionEvent::Stopped(_, _, _) => todo!(),
        }
    }

    if event_collision_source != Entity::PLACEHOLDER
        && event_collision_target != Entity::PLACEHOLDER
    {
        event_writer.send(ProjectileCollisionEvent {
            collision_source: event_collision_source,
            collision_target: event_collision_target,
        });
    }
}
