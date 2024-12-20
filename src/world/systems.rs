use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use super::{component::MapRepresentation, resource::MapImg};

pub fn spawn_world(mut commands: Commands, img_res: Res<MapImg>, map: Res<MapRepresentation>) {
    commands.spawn(Sprite::from_image(img_res.0.clone()));

    for collision in &map.collisions {
        commands.spawn((
            RigidBody::Static,
            Collider::rectangle(collision.rect.x, collision.rect.y),
            Transform::from_xyz(collision.position.x, collision.position.y, 1.),
        ));
    }
    // commands.spawn((
    //     RigidBody::Static,
    //     Collider::rectangle(430., 300.),
    //     Transform::from_xyz(-345., -230., 1.),
    // ));
    // commands.spawn((
    //     RigidBody::Static,
    //     Collider::rectangle(430., 300.),
    //     Transform::from_xyz(345., -230., 1.),
    // ));
    // commands.spawn((
    //     RigidBody::Static,
    //     Collider::rectangle(590., 10.),
    //     Transform::from_xyz(0., 160., 1.),
    // ));
    // commands.spawn((
    //     RigidBody::Static,
    //     Collider::rectangle(320., 10.),
    //     Transform::from_xyz(550., 35., 1.),
    // ));
    // commands.spawn((
    //     RigidBody::Static,
    //     Collider::rectangle(320., 10.),
    //     Transform::from_xyz(-550., 35., 1.),
    // ));
}
