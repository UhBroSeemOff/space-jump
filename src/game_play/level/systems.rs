use bevy::{
    prelude::{AssetServer, Commands, Entity, EventReader, EventWriter, Name, Query, Res, With},
    scene::DynamicSceneBundle,
    utils::default,
};

use super::{
    components::{Level, LevelBundle},
    events::{LoadLevelEvent, UnloadLevelEvent},
};

pub fn load_level(
    mut events: EventReader<LoadLevelEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if events.is_empty() {
        return;
    }

    let load_event = events.iter().nth(0);

    let mut scene = match load_event {
        Some(event) => commands.spawn(DynamicSceneBundle {
            scene: asset_server.load(event.scene_asset_path),
            ..default()
        }),
        None => return,
    };

    scene.insert(LevelBundle {
        name: Name::new("Test level"),
        level: Level,
    });
}

pub fn unload_level(
    mut events: EventReader<UnloadLevelEvent>,
    levels: Query<Entity, With<Level>>,
    mut commands: Commands,
) {
    if events.is_empty() {
        return;
    }

    events.clear();

    levels
        .iter()
        .for_each(|level| commands.entity(level).despawn())
}

// TODO: Remove test system
pub fn load_test_scene(mut event: EventWriter<LoadLevelEvent>) {
    event.send(LoadLevelEvent {
        scene_asset_path: "./assets/scenes/test.scn.ron",
    });
}
