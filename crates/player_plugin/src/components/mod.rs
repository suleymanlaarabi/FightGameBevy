pub mod config;
use animation_plugin::Animation2d;
use avian2d::prelude::*;
use bevy::prelude::*;
use config_plugin::player::PlayerConfig;
use grounded_plugin::GroundDetector;
use gun_plugin::components::Gun;
use slide_system::{SlideDetector, SlidingAllowed};

#[derive(Component, Default)]
#[require(Sprite)]
pub struct Player;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct PlayerLife(pub f32);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct PlayerDamage(pub f32);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct PlayerRepulse(pub f32, pub f32);

#[derive(Component)]
pub struct PlayerCollision;

impl Default for PlayerLife {
    fn default() -> Self {
        Self(100.)
    }
}

// IMPLEMENTATION
impl Player {
    pub fn new(
        image: &Handle<Image>,
        atlas_layout: &Handle<TextureAtlasLayout>,
        x: f32,
        y: f32,
    ) -> impl Bundle {
        let mut sprite = Sprite::from_image(image.clone());
        sprite.texture_atlas = Some(TextureAtlas {
            index: 0,
            layout: atlas_layout.clone(),
            ..default()
        });
        let mut transform = Transform::from_scale(Vec3::splat(3.));
        transform.translation = Vec3::new(x, y, 1.);
        (sprite, Player::default(), transform)
    }

    pub fn base_bundle() -> impl Bundle {
        return (
            LockedAxes::ROTATION_LOCKED,
            RigidBody::Dynamic,
            GravityScale(100.),
            LinearVelocity::ZERO,
            Friction::new(-0.45),
            PlayerLife(290.),
            Animation2d::new(0.2, 0, 3),
        );
    }

    pub fn full(
        image: &Handle<Image>,
        atlas_layout: &Handle<TextureAtlasLayout>,
        x: f32,
        controll: impl Bundle,
        features: impl Bundle,
    ) -> impl Bundle {
        (
            Player::new(image, atlas_layout, x, 350.),
            Self::base_bundle(),
            controll,
            features,
        )
    }
}

pub fn collision_detector(offset: f32) -> impl Bundle {
    (
        Collider::rectangle(10., 9.),
        Sensor,
        Transform::from_xyz(0., offset, 0.),
        GroundDetector,
    )
}

pub fn spawn_player(commands: &mut Commands, config: &PlayerConfig, controll: impl Bundle) {
    commands
        .spawn((
            Player::full(&config.image, &config.atlas, -250., controll, ()),
            config.animation_config.clone(),
            config.clone(),
            Gun {
                frequency: 0.4,
                image: None,
            },
        ))
        .insert_if((SlidingAllowed, SlideDetector), || config.features.slide)
        .with_children(|parent| {
            parent.spawn(collision_detector(config.ground_x));
            parent.spawn((
                Collider::capsule(config.collider_size.x, config.collider_size.y),
                PlayerCollision,
                Transform::from_xyz(config.collider_offset.x, config.collider_offset.y, 0.),
            ));
        });
}
