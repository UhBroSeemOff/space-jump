use bevy::prelude::*;

use crate::resources::{ApplicationState, GameState};

pub fn toggle_game_status(
    mut game_status_state: ResMut<NextState<GameState>>,
    simulation_state: Res<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let concrete_simultation_state = simulation_state.0;

        match concrete_simultation_state {
            GameState::Game => game_status_state.set(GameState::Game),
            GameState::GameOver => game_status_state.set(GameState::GameOver),
            GameState::PauseMenu => game_status_state.set(GameState::PauseMenu),
        }
    }
}