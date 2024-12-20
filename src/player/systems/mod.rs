pub mod animation;
pub mod attack;
pub mod attack_collision;
pub mod fail;

use avian2d::prelude::*;
use bevy::prelude::*;
use grounded_plugin::{GroundDetector, IsGrounded};
use slide_system::{Sliding, SlidingAllowed};

use crate::resources::{AdventurerAtlasLayout, PlayerTileSheet};

use super::components::{Controllable, Player, PlayerLife};

pub const PLAYER_VELOCITY_X: f32 = 300.0;
pub const PLAYER_JUMP_FORCE: f32 = 530.0;

pub fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &Controllable)>,
) {
    for (mut velocity, controllable) in query.iter_mut() {
        velocity.x = 0.;
        if keys.pressed(controllable.right) {
            velocity.x = PLAYER_VELOCITY_X;
        } else if keys.pressed(controllable.left) {
            velocity.x = -PLAYER_VELOCITY_X;
        }
    }
}

pub fn boost_velocity(mut query: Query<&mut LinearVelocity, With<Sliding>>) {
    for mut velocity in &mut query {
        velocity.x *= 1.5;
    }
}

pub fn handle_jump(
    mut query: Query<
        (&mut LinearVelocity, &Controllable),
        (With<Player>, With<IsGrounded>, With<Controllable>),
    >,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut velocity, controllable) in &mut query {
        if keys.just_pressed(controllable.up) {
            velocity.y = PLAYER_JUMP_FORCE;
        }
    }
}

pub fn handle_slide(
    mut query: Query<(Entity, &Controllable), (With<SlidingAllowed>, Without<Sliding>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (entity, controllable) in &mut query {
        if keys.pressed(controllable.down) {
            commands.entity(entity).insert(Sliding::default());
        }
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
        .spawn((
            Player::full(&image, &atlas_layout, -250., Controllable::default()),
            PlayerLife(290.),
        ))
        .with_children(|parent| {
            parent.spawn(collision_detector());
        });
}
