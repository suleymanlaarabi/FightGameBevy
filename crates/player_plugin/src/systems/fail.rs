use bevy::prelude::*;

use crate::components::{Player, PlayerDamage};

pub fn handle_fail(
    mut query: Query<(Entity, &mut Transform), With<Player>>,
    mut commands: Commands,
) {
    for (entity, mut transform) in &mut query {
        if transform.translation.y <= -300. {
            commands.entity(entity).insert(PlayerDamage(90.));
            transform.translation = Vec3::new(200., 410., 1.);
        }
    }
}
