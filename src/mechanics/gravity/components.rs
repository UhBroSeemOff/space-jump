use bevy::prelude::{Bundle, Component};
use bevy_rapier2d::prelude::ColliderMassProperties;

#[derive(Bundle)]
pub struct GravityBundle {
    pub subject_info: GravitySubject,
    pub mass: ColliderMassProperties,
}

#[derive(Component)]
pub struct GravitySubject {
    pub is_sensitive: bool,
}
