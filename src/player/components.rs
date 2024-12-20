use animation_plugin::Animation2d;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(
    Animation2d(|| Animation2d::new(0.2, 0, 3)),
    Sprite
)]
pub struct Player;
#[derive(Component)]
pub struct PlayerLife(pub f32);

impl Default for PlayerLife {
    fn default() -> Self {
        Self(100.)
    }
}

#[derive(Component)]
pub struct Controllable {
    pub top: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    pub hit: KeyCode,
}
