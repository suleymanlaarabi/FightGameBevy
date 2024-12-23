use bevy::prelude::*;

use crate::components::{Gun, GunFireAllowed, LastFireTime};

pub fn schedule_gun(
    mut query: Query<(Entity, &mut LastFireTime), With<Gun>>,
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

pub fn fire(gun: &Gun, entity: Entity, bullet: impl Bundle, commands: &mut Commands) {
    commands
        .entity(entity)
        .remove::<GunFireAllowed>()
        .insert(LastFireTime(Timer::from_seconds(
            gun.frequency,
            TimerMode::Once,
        )));
    commands.spawn(bullet);
}
