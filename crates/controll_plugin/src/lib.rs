use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Controllable {
    pub top: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    pub hit: KeyCode,
}

#[derive(Component)]
pub struct GamepadControlled(pub Entity);

#[derive(Component)]
pub enum ConnectedControll {
    Gamepad(Entity),
    Keyboard(Controllable),
}

impl Controllable {
    pub fn new(
        left: KeyCode,
        right: KeyCode,
        up: KeyCode,
        down: KeyCode,
        hit: KeyCode,
        top: KeyCode,
    ) -> Self {
        Self {
            left,
            right,
            up,
            down,
            hit,
            top,
        }
    }
}

impl Default for Controllable {
    fn default() -> Self {
        Self::new(
            KeyCode::KeyA,
            KeyCode::KeyD,
            KeyCode::Space,
            KeyCode::ShiftLeft,
            KeyCode::KeyX,
            KeyCode::KeyW,
        )
    }
}
