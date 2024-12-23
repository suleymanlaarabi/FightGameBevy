use bevy::prelude::*;

use crate::components::{GunFireAllowed, LastFireTime};

pub fn schedule_gun(
    mut query: Query<(Entity, &mut LastFireTime)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let delta = time.delta();
    for (entity, mut timer) in &mut query {
        timer.tick(delta);
        if timer.just_finished() {
            commands
                .entity(entity)
                .remove::<LastFireTime>()
                .insert(GunFireAllowed);
        }
    }
}
