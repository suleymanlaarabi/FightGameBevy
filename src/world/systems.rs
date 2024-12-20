use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use super::{
    component::{MapParent, WorldCollision, WorldSpawnRequest},
    WorldMaps,
};

pub fn spawn_world(mut commands: Commands) {
    commands.spawn(WorldSpawnRequest(0));
}

pub fn world_spawn_handle(
    query: Query<(Entity, &WorldSpawnRequest)>,
    maps: Res<WorldMaps>,
    mut commands: Commands,
    query_parent: Query<Entity, With<MapParent>>,
    query_collisions: Query<Entity, With<WorldCollision>>,
) {
    for (entity, request) in &query {
        // DESPAWN OLD MAP
        for entity in &query_parent {
            commands.entity(entity).despawn();
        }
        for entity in &query_collisions {
            commands.entity(entity).despawn();
        }
        // SPAWN NEW MAP
        let img = &maps[request.0].0;
        let collisions = &maps[request.0].1;
        commands.spawn((Sprite::from_image(img.clone()), MapParent));

        for collision in collisions {
            commands.spawn((
                RigidBody::Static,
                Collider::rectangle(collision.rect.x, collision.rect.y),
                Transform::from_xyz(collision.position.x, collision.position.y, 1.),
                WorldCollision,
            ));
        }
        commands.entity(entity).despawn();
    }
}
