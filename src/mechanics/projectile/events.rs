use bevy::prelude::Entity;

pub struct ProjectileCollisionEvent {
    pub collision_source: Entity, // Who collided
    pub collision_target: Entity, // With whom collision occurred
}
