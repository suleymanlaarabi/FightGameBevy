use std::collections::HashMap;

use bevy::{ecs::system::SystemId, prelude::*};
use systems::game_button_interaction;

pub mod builder;
pub mod components;
pub mod styles;
mod systems;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, game_button_interaction);
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct ButtonMapSystems(pub HashMap<String, SystemId>);
