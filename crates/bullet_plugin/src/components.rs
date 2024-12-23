use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Event)]
pub struct BulletHitEvent(pub Entity);

pub fn default_bullet_velocity(reverse: bool) -> LinearVelocity {
    let mut velocity = LinearVelocity::ZERO;
    velocity.x = 1000.;
    if reverse {
        velocity.x = -velocity.x;
    }
    velocity
}

#[derive(Component, Default)]
#[require(
    Transform,
    RigidBody(|| RigidBody::Dynamic),
    LinearVelocity(|| default_bullet_velocity(false)),
    Collider(|| Collider::rectangle(60., 10.)),
    GravityScale(|| GravityScale(0.)),
    Sensor
)]
pub struct Bullet;
