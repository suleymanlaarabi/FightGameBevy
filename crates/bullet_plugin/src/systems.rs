use avian2d::prelude::CollisionStarted;
use bevy::prelude::*;

use crate::components::{Bullet, BulletHitEvent};

pub fn bullet_collision_check(
    query: Query<Entity, With<Bullet>>,
    mut collision_events: EventReader<CollisionStarted>,
    mut bullet_event_writer: EventWriter<BulletHitEvent>,
) {
    for event in collision_events.read() {
        for entity_bullet in &query {
            if event.0 == entity_bullet {
                bullet_event_writer.send(BulletHitEvent(event.1));
            } else if event.1 == entity_bullet {
                bullet_event_writer.send(BulletHitEvent(event.0));
            }
        }
    }
}
