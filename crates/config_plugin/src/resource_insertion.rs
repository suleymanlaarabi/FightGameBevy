use std::fs::read_dir;

use bevy::prelude::*;

use crate::{game_component::PlayerConfig, json_repr::PlayerSpriteConfig};

#[derive(Resource)]
pub struct PlayersConfig(pub Vec<PlayerConfig>);

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

impl PlayerConfig {
    pub fn new(world: &mut World, config: PlayerSpriteConfig) -> Self {
        let image = world.resource::<AssetServer>().load(&config.tiles.img_path);
        let tiles = &config.tiles;

        Self {
            atlas: world
                .resource::<AssetServer>()
                .add(TextureAtlasLayout::from_grid(
                    UVec2::new(tiles.height, tiles.width),
                    tiles.col,
                    tiles.row,
                    None,
                    None,
                )),
            image,
            preview: config.tiles.preview,
            animation_config: config.animation,
            collider_size: config.tiles.collider_size,
            collider_offset: config.tiles.collider_offset,
            ground_x: config.tiles.ground_x,
            features: config.features,
        }
    }
}
