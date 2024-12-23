use bevy::prelude::*;
use bullet_plugin::components::{Bullet, BulletHitEvent, default_bullet_velocity};
use controll_plugin::{Controllable, GamepadControlled};
use gun_plugin::{
    components::{Gun, GunFireAllowed},
    systems::fire,
};
use player_plugin::components::{Player, PlayerCollision, PlayerDamage};

fn spawn_bullet(
    player_transform: &Transform,
    player_sprite: &Sprite,
    gun: &Gun,
    entity: Entity,
    commands: &mut Commands,
) {
    let mut transform = Transform::default();
    transform.translation = player_transform.translation;
    if player_sprite.flip_x {
        transform.translation.x -= 60.;
    } else {
        transform.translation.x += 60.;
    }
    let velocity = default_bullet_velocity(player_sprite.flip_x);
    fire(gun, entity, (Bullet, transform, velocity), commands);
}

pub fn handle_player_fire(
    query: Query<
        (&Gun, Entity, &Controllable, &Transform, &Sprite),
        (With<Player>, With<GunFireAllowed>),
    >,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (gun, entity, controll, player_transform, sprite) in &query {
        if keys.pressed(controll.fire) {
            spawn_bullet(player_transform, sprite, gun, entity, &mut commands);
        }
    }
}

pub fn handle_player_gamepad_fire(
    query: Query<
        (&Gun, Entity, &GamepadControlled, &Transform, &Sprite),
        (With<Player>, With<GunFireAllowed>),
    >,
    gamepads: Query<(Entity, &Gamepad)>,
    mut commands: Commands,
) {
    for (gun, entity, controll, player_transform, sprite) in &query {
        for (gamepad_entity, gamepad) in &gamepads {
            if gamepad_entity == controll.0 {
                if gamepad.pressed(GamepadButton::RightTrigger) {
                    spawn_bullet(player_transform, sprite, gun, entity, &mut commands);
                }
            }
        }
    }
}

pub fn handle_player_touched(
    mut events_hit: EventReader<BulletHitEvent>,
    query_player: Query<(Entity, &Parent), With<PlayerCollision>>,
    mut commands: Commands,
) {
    for event in events_hit.read() {
        for (entity, parent) in &query_player {
            if event.0 == entity {
                let entity = parent.get();
                commands.entity(entity).insert(PlayerDamage(30.));
            }
        }
    }
}
