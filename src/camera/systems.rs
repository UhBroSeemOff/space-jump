use bevy::prelude::{Camera2dBundle, Commands, Query, Transform, With, Without};

use super::components::{CameraTarget, MainCamera};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle::default()));
}

pub fn follow_target(
    mut camera_query: Query<&mut Transform, (Without<CameraTarget>, With<MainCamera>)>,
    target_query: Query<&Transform, With<CameraTarget>>,
) {
    let mut camera = camera_query.single_mut();

    if let Ok(target) = target_query.get_single() {
        let mut temp_translation = target.translation.clone();
        temp_translation.z = 0.0; // Since there is only 2 dimensions - Z is redundant
        camera.translation = temp_translation;
    }
}
