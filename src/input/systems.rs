use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput},
    prelude::{EventReader, EventWriter, Res},
};

use super::{events::InputActionEvent, resources::KeyBindings};

pub fn handle_keyboard_input(
    mut key_input_event: EventReader<KeyboardInput>,
    mut action_input_event: EventWriter<InputActionEvent>,
    bindings: Res<KeyBindings>,
) {
    if key_input_event.is_empty() {
        return;
    }

    key_input_event
        .iter()
        .filter_map(|input| input.key_code.ok_or(-1).ok())
        .filter(|pressed_key| bindings.keyboard().contains_key(pressed_key))
        .filter_map(|key| bindings.keyboard().get(&key).ok_or(-1).ok())
        .for_each(|action| action_input_event.send(InputActionEvent::new(action.to_owned())));
}

pub fn handle_mouse_input(
    mut events: EventReader<MouseButtonInput>,
    mut action_input_event: EventWriter<InputActionEvent>,
    bindings: Res<KeyBindings>,
) {
    if events.is_empty() {
        return;
    }

    events
        .iter()
        .map(|event| event.button)
        .filter(|button| bindings.mouse().contains_key(button))
        .filter_map(|button| bindings.mouse().get(&button).ok_or(-1).ok())
        .for_each(|action| action_input_event.send(InputActionEvent::new(action.to_owned())));
}

// TODO: Remove test system
pub fn test_keyboard_system(mut event: EventReader<InputActionEvent>) {
    if event.is_empty() {
        return;
    }

    event
        .iter()
        .for_each(|event| println!("Pressed: {:?}", event.action()));
}
