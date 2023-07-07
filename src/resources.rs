use bevy::prelude::States;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ApplicationState {
    #[default]
    MainMenu,
    LevelPick,
    Game,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MainMenuState {
    #[default]
    MainScreen,
    Settings,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Game,
    GameOver,
    PauseMenu,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PauseMenuState {
    #[default]
    Pause,
    Settings,
    MainMenu,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SettingsState {
    #[default]
    Audio,
    Video,
}