use bevy::prelude::*;
use config_plugin::resource_insertion::PlayersConfig;

pub struct PlayerResourcePlugin;

impl Plugin for PlayerResourcePlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        let players_config = PlayersConfig::from_world(world);
        app.insert_resource(players_config);
    }
}
