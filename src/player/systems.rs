use animation_plugin::Animation2d;
use attack_plugin::Attack;
use avian2d::prelude::*;
use bevy::prelude::*;
use grounded_plugin::{GroundDetector, IsGrounded};
use slide_system::{Sliding, SlidingAllowed};

use crate::resources::{AdventurerAtlasLayout, PlayerTileSheet};

use super::Player;

const PLAYER_VELOCITY_X: f32 = 300.0;
const PLAYER_JUMP_FORCE: f32 = 530.0;

pub fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut Player,
        Option<&Sliding>,
        &mut LinearVelocity,
        &mut Sprite,
    )>,
) {
    let (mut player, in_slide, mut velocity, mut sprite) = query.single_mut();
    velocity.x = 0.;
    if keys.pressed(KeyCode::KeyD) {
        sprite.flip_x = false;
        velocity.x = PLAYER_VELOCITY_X;
        player.is_move = true;
    } else if keys.pressed(KeyCode::KeyA) {
        sprite.flip_x = true;
        velocity.x = -PLAYER_VELOCITY_X;
        player.is_move = true;
    } else {
        player.is_move = false;
    }
    if in_slide.is_some() {
        velocity.x *= 1.5;
    }
}

pub fn handle_animation(
    mut query: Query<
        (
            &Player,
            &LinearVelocity,
            Option<&Sliding>,
            &mut Animation2d,
            Option<&IsGrounded>,
            Option<&Attack>,
        ),
        With<Player>,
    >,
) {
    for (player, velocity, in_slide, mut animation, is_grounded, attack) in &mut query {
        if !is_grounded.is_some() {
            if velocity.y > 0. {
                animation.set_animation(16, 17, 0.5);
            } else {
                animation.set_animation(22, 23, 0.2);
            }
        } else if in_slide.is_some() {
            animation.set_animation(24, 25, 0.2);
        } else if attack.is_some() {
            animation.set_animation(40, 52, 0.1);
        } else if player.is_move {
            animation.set_animation(10, 12, 0.2);
        } else {
            animation.set_animation(0, 3, 0.2);
        }
    }
}

pub fn handle_jump(
    mut query: Query<&mut LinearVelocity, (With<Player>, With<IsGrounded>)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut velocity in &mut query {
        if keys.just_pressed(KeyCode::Space) {
            velocity.y = PLAYER_JUMP_FORCE;
        }
    }
}

pub fn handle_slide(
    mut query: Query<Entity, (With<SlidingAllowed>, Without<Sliding>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for entity in &mut query {
        if keys.pressed(KeyCode::ShiftLeft) {
            println!("INSERT SLIDING");
            commands.entity(entity).insert(Sliding::default());
        }
    }
}

pub fn check_attack(
    query: Query<Entity, (With<Player>, Without<Attack>)>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for entity in &query {
        if keys.just_pressed(KeyCode::KeyX) {
            commands.entity(entity).insert(Attack::default());
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    image: Res<PlayerTileSheet>,
    atlas_layout: Res<AdventurerAtlasLayout>,
) {
    commands
        .spawn((
            Player::new(&image, &atlas_layout),
            Collider::capsule(8., 15.),
            LockedAxes::ROTATION_LOCKED,
            RigidBody::Dynamic,
            GravityScale(100.),
            LinearVelocity::ZERO,
            Friction::new(-0.45),
            SlidingAllowed,
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::rectangle(10., 10.),
                Sensor,
                Transform::from_xyz(0., -15., 0.),
                GroundDetector,
            ));
        });
}
