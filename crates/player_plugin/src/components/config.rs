// use std::fs::read_to_string;

// use bevy::prelude::*;
// use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize)]
// pub struct PlayerTiles {
//     pub img_path: String,
//     pub preview: u32,
//     pub col: u32,
//     pub row: u32,
//     pub width: u32,
//     pub height: u32,
//     pub collider_offset: Vec2,
//     pub collider_size: Vec2,
//     pub ground_x: f32,
// }

// #[derive(Component, Deserialize, Serialize, Clone)]
// pub struct PlayerAnimationConfig {
//     pub idle: Vec3,
//     pub attack: Vec3,
//     pub attack2: Vec3,
//     pub top_attack: Vec3,
//     pub movement: Vec3,
//     pub jump: Vec3,
//     pub fail: Vec3,
//     pub slide: Vec3,
// }

// #[derive(Component, Deserialize, Serialize)]
// pub struct PlayerSpriteConfig {
//     pub tiles: PlayerTiles,
//     pub animation: PlayerAnimationConfig,
//     pub features: PlayerConfigFeatures,
// }

// #[derive(Deserialize, Serialize, Clone)]
// pub struct PlayerConfigFeatures {
//     pub slide: bool,
// }

// #[derive(Resource)]
// pub struct PlayersConfig(pub Vec<PlayerConfig>);

// #[derive(Component, Clone)]
// pub struct PlayerConfig {
//     pub atlas: Handle<TextureAtlasLayout>,
//     pub image: Handle<Image>,
//     pub preview: u32,
//     pub animation_config: PlayerAnimationConfig,
//     pub collider_offset: Vec2,
//     pub collider_size: Vec2,
//     pub ground_x: f32,
//     pub features: PlayerConfigFeatures,
// }

// impl PlayerConfig {
//     pub fn new(world: &mut World, config: PlayerSpriteConfig) -> Self {
//         let image = world.resource::<AssetServer>().load(&config.tiles.img_path);
//         let tiles = &config.tiles;

//         Self {
//             atlas: world
//                 .resource::<AssetServer>()
//                 .add(TextureAtlasLayout::from_grid(
//                     UVec2::new(tiles.height, tiles.width),
//                     tiles.col,
//                     tiles.row,
//                     None,
//                     None,
//                 )),
//             image,
//             preview: config.tiles.preview,
//             animation_config: config.animation,
//             collider_size: config.tiles.collider_size,
//             collider_offset: config.tiles.collider_offset,
//             ground_x: config.tiles.ground_x,
//             features: config.features,
//         }
//     }
// }

// impl PlayerSpriteConfig {
//     pub fn from_file(path: impl Into<String>) -> Self {
//         let file_content = read_to_string(path.into()).expect("File player not found");
//         serde_json::from_str(&file_content).expect("file syntaxe not valid")
//     }
// }
