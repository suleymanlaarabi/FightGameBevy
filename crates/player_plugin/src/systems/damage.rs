use bevy::prelude::*;

use crate::components::{PlayerDamage, PlayerLife};

pub fn apply_damage(
    mut query: Query<(Entity, &mut PlayerLife, &PlayerDamage)>,
    mut commands: Commands,
) {
    for (entity, mut player_life, damage) in &mut query {
        player_life.0 -= damage.0;
        if player_life.0 <= 0. {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        commands.entity(entity).remove::<PlayerDamage>();
    }
}
