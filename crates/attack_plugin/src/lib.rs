use bevy::prelude::*;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, during_attack);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Attack(Timer);

impl Default for Attack {
    fn default() -> Self {
        Self(Timer::from_seconds(1.3, TimerMode::Once))
    }
}

fn during_attack(mut query: Query<(Entity, &mut Attack)>, time: Res<Time>, mut commands: Commands) {
    let delta = time.delta();
    for (entity, mut attack) in &mut query {
        attack.tick(delta);
        if attack.just_finished() {
            commands.entity(entity).remove::<Attack>();
        }
    }
}
