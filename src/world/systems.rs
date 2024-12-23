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
        if request.0 < 0 {
            return;
        }
        // SPAWN NEW MAP
        let id = request.0 as usize;
        let img = &maps[id].0;
        let collisions = &maps[id].2;
        let audio = &maps[id].1;
        let mut command = commands.spawn((Sprite::from_image(img.clone()), MapParent));
        if let Some(audio) = audio {
            command.insert(AudioPlayer::new(audio.clone()));
        }
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
