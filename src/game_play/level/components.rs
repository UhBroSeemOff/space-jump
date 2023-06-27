use bevy::{
    prelude::{Bundle, Component, Name, ReflectComponent},
    reflect::Reflect,
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Level;

#[derive(Bundle)]
pub struct LevelBundle {
    pub name: Name,
    pub level: Level,
}
