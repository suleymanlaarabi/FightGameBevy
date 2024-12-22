use bevy::prelude::*;

use crate::json_repr::{PlayerAnimationConfig, PlayerConfigFeatures};

#[derive(Component, Clone)]
pub struct PlayerConfig {
    pub atlas: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
    pub preview: u32,
    pub animation_config: PlayerAnimationConfig,
    pub collider_offset: Vec2,
    pub collider_size: Vec2,
    pub ground_x: f32,
    pub features: PlayerConfigFeatures,
}
