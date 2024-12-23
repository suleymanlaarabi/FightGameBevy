use bevy::prelude::*;
use systems::schedule_gun;

pub mod components;
pub mod systems;

pub struct GameGunPlugin;

impl Plugin for GameGunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, schedule_gun);
    }
}
