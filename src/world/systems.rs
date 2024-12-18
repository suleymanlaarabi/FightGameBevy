use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use super::resource::MapImg;

pub fn spawn_world(mut commands: Commands, img_res: Res<MapImg>) {
    commands.spawn(Sprite::from_image(img_res.0.clone()));
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(430., 300.),
        Transform::from_xyz(-345., -230., 1.),
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(430., 300.),
        Transform::from_xyz(345., -230., 1.),
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(590., 10.),
        Transform::from_xyz(0., 160., 1.),
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(320., 10.),
        Transform::from_xyz(550., 35., 1.),
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(320., 10.),
        Transform::from_xyz(-550., 35., 1.),
    ));
}
