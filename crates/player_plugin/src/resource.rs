use std::fs::read_dir;

use bevy::prelude::*;

use crate::components::config::{PlayerConfig, PlayerSpriteConfig, PlayersConfig};

pub struct PlayerResourcePlugin;

impl Plugin for PlayerResourcePlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        let players_config = PlayersConfig::from_world(world);
        app.insert_resource(players_config);
    }
}

impl FromWorld for PlayersConfig {
    fn from_world(world: &mut World) -> Self {
        let mut players_config = Vec::new();
        let dir = read_dir("assets/players").expect("Players dir not found");
        for entry in dir {
            let entry = entry.expect("error file in players dir");
            let path = entry.path();
            let config = PlayerSpriteConfig::from_file(path.to_str().unwrap());
            players_config.push(PlayerConfig::new(world, config));
        }
        Self(players_config)
    }
}
