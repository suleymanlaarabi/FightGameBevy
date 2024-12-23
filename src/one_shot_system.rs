use bevy::prelude::*;
use one_shot_system::GameOneShotSystemBuilder;

pub fn register_one_shot_system(app: &mut App) {
    GameOneShotSystemBuilder::new(app.world_mut());
}
