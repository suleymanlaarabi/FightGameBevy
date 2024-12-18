use animation_plugin::Animation2d;
use bevy::prelude::*;
use systems::{
    check_attack, handle_animation, handle_jump, handle_slide, move_player, spawn_player,
};

use crate::resources::{AdventurerAtlasLayout, PlayerTileSheet};

mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                move_player.before(handle_animation).before(handle_slide),
                (handle_jump, handle_animation, handle_slide, check_attack),
            ),
        );
    }
}

#[derive(Component, Default)]
#[require(
    Animation2d(|| Animation2d::new(0.2, 0, 3)),
    Sprite
)]
pub struct Player {
    is_move: bool,
}

impl Player {
    pub fn new(
        image: &Res<PlayerTileSheet>,
        atlas_layout: &Res<AdventurerAtlasLayout>,
    ) -> impl Bundle {
        let mut sprite = Sprite::from_image(image.0.clone());
        sprite.texture_atlas = Some(TextureAtlas {
            index: 0,
            layout: atlas_layout.0.clone(),
            ..default()
        });
        let mut transform = Transform::from_scale(Vec3::splat(3.));
        transform.translation.z = 1.;
        transform.translation.x = -250.;
        transform.translation.y = 150.;
        (sprite, Player::default(), transform)
    }
}
