pub mod events;
pub mod resources;
mod systems;

use bevy::prelude::{App, KeyCode, MouseButton, Plugin};

use self::{
    events::InputActionEvent,
    resources::KeyBindings,
    systems::{handle_keyboard_input, handle_mouse_input, test_keyboard_system},
};

#[derive(Default, Debug, Clone, Copy)]
pub enum InputAction {
    #[default]
    Default,
    Restart,
    Throw,
    Jump,
    Hook,
    Pull,
}

const MOUSE_BINDINGS: [(MouseButton, InputAction); 2] = [
    (MouseButton::Left, InputAction::Hook),
    (MouseButton::Right, InputAction::Pull),
];

const KEYBOARD_BINDINGS: [(KeyCode, InputAction); 3] = [
    (KeyCode::R, InputAction::Restart),
    (KeyCode::E, InputAction::Throw),
    (KeyCode::Space, InputAction::Jump),
];

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputActionEvent>()
            .insert_resource(KeyBindings::new())
            .add_system(handle_keyboard_input)
            .add_system(handle_mouse_input)
            .add_system(test_keyboard_system);
    }
}
