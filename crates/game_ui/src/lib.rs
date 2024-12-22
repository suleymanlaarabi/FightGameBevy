use std::collections::HashMap;

use bevy::{ecs::system::SystemId, prelude::*};
use components::GameButtonPlugin;

pub mod builder;
pub mod components;
pub mod styles;
mod systems;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameButtonPlugin);
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct ButtonMapSystems(pub HashMap<String, SystemId>);
