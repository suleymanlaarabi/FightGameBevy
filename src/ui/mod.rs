use std::collections::HashMap;

use ::game_ui::ButtonMapSystems;
use bevy::prelude::*;
use game_ui::GameMenuPlugin;
use main_menu::MainMenuPlugin;

use crate::{
    world::{component::WorldSpawnRequest, WorldMaps},
    GameState,
};
pub mod game_ui;
pub mod main_menu;
pub mod settings;

pub struct CustomUiPlugin;

impl Plugin for CustomUiPlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        insert_system(world);
        app.add_plugins((GameMenuPlugin, MainMenuPlugin));
    }
}

fn insert_system(world: &mut World) {
    let mut map = ButtonMapSystems(HashMap::new());

    map.insert(
        "on_click_exit_game".into(),
        world.register_system(|mut state: ResMut<NextState<GameState>>| {
            state.set(GameState::StartMenu);
        }),
    );
    map.insert(
        "on_click_settings_game".into(),
        world.register_system(|mut state: ResMut<NextState<GameState>>| {
            state.set(GameState::Settings);
        }),
    );
    map.insert(
        "on_click_play".into(),
        world.register_system(|mut state: ResMut<NextState<GameState>>| {
            state.set(GameState::InFight);
        }),
    );
    map.insert(
        "refresh_maps".into(),
        world.register_system(|world: &mut World| {
            let new = WorldMaps::from_world(world);
            let mut res = world.resource_mut::<WorldMaps>();
            res.0 = new.0;
            world.spawn(WorldSpawnRequest(0));
        }),
    );
    world.insert_resource(map);
}
