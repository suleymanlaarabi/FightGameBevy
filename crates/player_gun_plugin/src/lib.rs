use bevy::prelude::*;
use systems::{handle_player_fire, handle_player_gamepad_fire, handle_player_touched};

pub mod systems;

pub struct PlayerGunPlugin;

impl Plugin for PlayerGunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_player_fire,
                handle_player_gamepad_fire,
                handle_player_touched,
            ),
        );
    }
}
