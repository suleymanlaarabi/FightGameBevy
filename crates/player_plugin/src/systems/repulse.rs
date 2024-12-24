use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::components::PlayerRepulse;

pub fn apply_player_repulse(
    mut query: Query<(Entity, &PlayerRepulse, &mut LinearVelocity, &Sprite)>,
    mut commands: Commands,
) {
    for (entity, repulse, mut velocity, sprite) in &mut query {
        let mut forcex = repulse.0;
        if sprite.flip_x {
            forcex = -forcex;
        }
        velocity.x += forcex;
        velocity.y += repulse.1;
        commands.entity(entity).remove::<PlayerRepulse>();
    }
}
