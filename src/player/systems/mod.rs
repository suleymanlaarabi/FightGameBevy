pub mod animation;
pub mod attack;
pub mod attack_collision;
pub mod fail;
pub mod movement;
pub mod slide;

use avian2d::prelude::*;
use bevy::prelude::*;
use grounded_plugin::GroundDetector;
use slide_system::Sliding;

use crate::resources::{AdventurerAtlasLayout, PlayerTileSheet};

use super::components::{Controllable, Player};

pub fn boost_velocity(mut query: Query<&mut LinearVelocity, With<Sliding>>) {
    for mut velocity in &mut query {
        velocity.x *= 1.5;
    }
}

pub fn despawn_player(query: Query<Entity, With<Player>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn collision_detector() -> impl Bundle {
    (
        Collider::rectangle(10., 3.),
        Sensor,
        Transform::from_xyz(0., -15., 0.),
        GroundDetector,
    )
}

pub fn spawn_player(
    mut commands: Commands,
    image: Res<PlayerTileSheet>,
    atlas_layout: Res<AdventurerAtlasLayout>,
) {
    commands
        .spawn(Player::full(
            &image,
            &atlas_layout,
            -250.,
            Controllable::default(),
        ))
        .with_children(|parent| {
            parent.spawn(collision_detector());
        });
}
