use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct BulletHitEvent(pub Entity);

#[derive(Component, Default)]
#[require(
    Transform,
    RigidBody(|| RigidBody::Dynamic),
    LinearVelocity(|| {
        let mut velocity = LinearVelocity::ZERO;
        velocity.x = 1000.;
        velocity
    }),
    Collider(|| Collider::rectangle(60., 10.)),
    GravityScale(|| GravityScale(0.)),
    Sensor
)]
pub struct Bullet;
