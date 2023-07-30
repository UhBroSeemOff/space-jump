use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Projectile {
    pub coordinate: Vec2,
    pub direction: Vec2,
}
