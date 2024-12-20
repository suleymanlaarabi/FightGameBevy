use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use grounded_plugin::IsGrounded;
use jump_plugin::Jump;

use crate::player::components::{Controllable, Player};

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

pub fn handle_jump(
    mut query: Query<(Entity, &Controllable), (With<Player>, With<IsGrounded>, With<Controllable>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (entity, controllable) in &mut query {
        if keys.just_pressed(controllable.up) {
            commands.entity(entity).insert(Jump(PLAYER_JUMP_FORCE));
        }
    }
}
