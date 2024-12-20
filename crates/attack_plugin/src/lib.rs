use avian2d::prelude::{Collider, Sensor};
use bevy::prelude::*;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, during_attack);
    }
}

#[derive(Component)]
pub struct Attack {
    pub timer: Timer,
    pub id: u8,
}

#[derive(Component)]
pub struct AttackSensor;

impl Attack {
    pub fn attack_with_collision(
        id: u8,
        duration: f32,
        zone: Collider,
        commands: &mut Commands,
        entity: Entity,
    ) -> impl Bundle {
        let mut transform = Transform::from_xyz(0., 0., 0.);
        if id == 6 {
            transform.translation.y += 15.;
        }
        commands
            .entity(entity)
            .insert(Attack {
                id,
                timer: Timer::from_seconds(duration, TimerMode::Once),
            })
            .with_children(|parent| {
                parent.spawn((zone, Sensor, AttackSensor, transform));
            });
    }
}

impl Default for Attack {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
            id: 0,
        }
    }
}

fn during_attack(
    mut query: Query<(Entity, &mut Attack)>,
    sensor_query: Query<(Entity, &Parent), With<AttackSensor>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let delta = time.delta();
    for (entity, mut attack) in &mut query {
        attack.timer.tick(delta);
        if attack.timer.just_finished() {
            commands.entity(entity).remove::<Attack>();
            for (child_entity, parent) in &sensor_query {
                if parent.get() == entity {
                    commands.entity(child_entity).despawn_recursive();
                }
            }
        }
    }
}
