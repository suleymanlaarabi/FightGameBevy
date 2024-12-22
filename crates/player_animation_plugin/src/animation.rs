use animation_plugin::Animation2d;
use attack_plugin::Attack;
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use grounded_plugin::IsGrounded;
use player_plugin::components::{Player, config::PlayerAnimationConfig};
use slide_system::Sliding;

pub fn handle_animation(
    mut query: Query<
        (
            &LinearVelocity,
            &mut Sprite,
            Option<&Sliding>,
            &mut Animation2d,
            Option<&IsGrounded>,
            Option<&Attack>,
            &PlayerAnimationConfig,
        ),
        With<Player>,
    >,
) {
    for (velocity, mut sprite, in_slide, mut animation, is_grounded, attack, config) in &mut query {
        if let Some(attack) = attack {
            match attack.id {
                0 => animation.set_vec_anim(config.attack),
                1 => animation.set_vec_anim(config.attack),
                6 => animation.set_vec_anim(config.top_attack),
                _ => animation.set_vec_anim(config.attack2),
            }
        } else if !is_grounded.is_some() {
            if velocity.y > -150. {
                animation.set_vec_anim(config.jump);
            } else {
                animation.set_vec_anim(config.fail);
            }
        } else if in_slide.is_some() {
            animation.set_vec_anim(config.slide);
        } else if velocity.x < -130. || velocity.x > 130. {
            animation.set_vec_anim(config.movement);
        } else {
            animation.set_vec_anim(config.idle);
        }
        if velocity.x < -50. {
            sprite.flip_x = true;
        } else if velocity.x > 50. {
            sprite.flip_x = false;
        }
    }
}
