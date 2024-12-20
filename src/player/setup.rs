use avian2d::prelude::*;
use bevy::prelude::*;
//use collision_trigger::CollisionCible;
use slide_system::SlidingAllowed;

use crate::resources::{AdventurerAtlasLayout, PlayerTileSheet};

use super::components::{Controllable, Player, PlayerLife};

impl Player {
    pub fn new(
        image: &Res<PlayerTileSheet>,
        atlas_layout: &Res<AdventurerAtlasLayout>,
        x: f32,
        y: f32,
    ) -> impl Bundle {
        let mut sprite = Sprite::from_image(image.0.clone());
        sprite.texture_atlas = Some(TextureAtlas {
            index: 0,
            layout: atlas_layout.0.clone(),
            ..default()
        });
        let mut transform = Transform::from_scale(Vec3::splat(3.));
        transform.translation = Vec3::new(x, y, 1.);
        (sprite, Player::default(), transform)
    }

    pub fn base_bundle() -> impl Bundle {
        (
            Collider::capsule(8., 15.),
            LockedAxes::ROTATION_LOCKED,
            RigidBody::Dynamic,
            GravityScale(100.),
            LinearVelocity::ZERO,
            Friction::new(-0.45),
            SlidingAllowed,
            PlayerLife(290.),
        )
    }

    pub fn full(
        image: &Res<PlayerTileSheet>,
        atlas_layout: &Res<AdventurerAtlasLayout>,
        x: f32,
        controll: impl Bundle,
    ) -> impl Bundle {
        (
            Player::new(image, atlas_layout, x, 220.),
            Self::base_bundle(),
            controll,
        )
    }
}

impl Controllable {
    pub fn new(
        left: KeyCode,
        right: KeyCode,
        up: KeyCode,
        down: KeyCode,
        hit: KeyCode,
        top: KeyCode,
    ) -> Self {
        Self {
            left,
            right,
            up,
            down,
            hit,
            top,
        }
    }
}

impl Default for Controllable {
    fn default() -> Self {
        Self::new(
            KeyCode::KeyA,
            KeyCode::KeyD,
            KeyCode::Space,
            KeyCode::ShiftLeft,
            KeyCode::KeyX,
            KeyCode::KeyW,
        )
    }
}
