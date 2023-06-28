use super::InputAction;

pub struct InputActionEvent {
    action: InputAction,
}

impl InputActionEvent {
    pub fn new(action: InputAction) -> Self {
        InputActionEvent { action }
    }

    pub fn action(&self) -> InputAction {
        self.action
    }
}
