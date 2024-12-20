use animation_plugin::Animation2d;
use attack_plugin::Attack;
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use grounded_plugin::IsGrounded;
use slide_system::Sliding;

use crate::player::components::Player;

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_animation);
    }
}

pub fn handle_animation(
    mut query: Query<
        (
            &LinearVelocity,
            &mut Sprite,
            Option<&Sliding>,
            &mut Animation2d,
            Option<&IsGrounded>,
            Option<&Attack>,
        ),
        With<Player>,
    >,
) {
    for (velocity, mut sprite, in_slide, mut animation, is_grounded, attack) in &mut query {
        if let Some(attack) = attack {
            match attack.id {
                0 => {
                    animation.set_animation(42, 48, 0.1);
                }
                1 => {
                    animation.set_animation(42, 48, 0.1);
                }
                6 => {
                    animation.set_animation(45, 48, 0.2);
                }
                _ => {
                    animation.set_animation(54, 58, 0.1);
                }
            }
        } else if !is_grounded.is_some() {
            if velocity.y > -250. {
                animation.set_animation(15, 21, 0.1);
            } else {
                animation.set_animation(22, 23, 0.2);
            }
        } else if in_slide.is_some() {
            animation.set_animation(23, 26, 0.2);
        } else if velocity.x < -130. || velocity.x > 130. {
            animation.set_animation(10, 12, 0.2);
        } else {
            animation.set_animation(0, 3, 0.2);
        }
        if velocity.x < -50. {
            sprite.flip_x = true;
        } else if velocity.x > 50. {
            sprite.flip_x = false;
        }
    }
}
