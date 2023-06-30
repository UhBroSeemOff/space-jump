use std::collections::HashMap;

use bevy::prelude::{KeyCode, MouseButton, Resource};

use super::{InputAction, KEYBOARD_BINDINGS, MOUSE_BINDINGS};

#[derive(Resource)]
pub struct KeyBindings {
    keyboard: HashMap<KeyCode, InputAction>,
    mouse: HashMap<MouseButton, InputAction>,
}

impl KeyBindings {
    pub fn new() -> Self {
        KeyBindings {
            keyboard: HashMap::from(KEYBOARD_BINDINGS),
            mouse: HashMap::from(MOUSE_BINDINGS),
        }
    }

    pub fn keyboard(&self) -> &HashMap<KeyCode, InputAction> {
        &self.keyboard
    }

    pub fn mouse(&self) -> &HashMap<MouseButton, InputAction> {
        &self.mouse
    }
}
