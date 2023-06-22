use bevy::{prelude::*, window::WindowMode};

fn main() {
    println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Fullscreen,
                title: "Space jump".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<ApplicationState>();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ApplicationState {
    #[default]
    MainMenu,
    Game,
    GameOver,
    PauseMenu,
}
