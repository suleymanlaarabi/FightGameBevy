use avian2d::prelude::CollisionStarted;
use bevy::prelude::*;

use crate::components::{Bullet, BulletHitEvent};

pub fn bullet_out_screen_check(
    query: Query<(Entity, &Transform), With<Bullet>>,
    mut commands: Commands,
) {
    for (entity_bullet, transform) in &query {
        if transform.translation.x > 650. || transform.translation.x < -650. {
            commands.entity(entity_bullet).despawn();
            continue;
        }
    }
}

pub fn bullet_collision_check(
    query: Query<Entity, With<Bullet>>,
    mut collision_events: EventReader<CollisionStarted>,
    mut bullet_event_writer: EventWriter<BulletHitEvent>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        for entity_bullet in &query {
            if event.0 == entity_bullet {
                bullet_event_writer.send(BulletHitEvent(event.1));
                commands.entity(entity_bullet).despawn();
            } else if event.1 == entity_bullet {
                commands.entity(entity_bullet).despawn();
                bullet_event_writer.send(BulletHitEvent(event.0));
            }
        }
    }
}
