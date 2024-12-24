use attack_plugin::AttackSensor;
use avian2d::prelude::*;
use bevy::prelude::*;
use player_plugin::components::{PlayerCollision, PlayerDamage, PlayerRepulse};

pub fn detect_attack_collision(
    player_query: Query<(Entity, &Parent), With<PlayerCollision>>,
    attack_query: Query<Entity, With<AttackSensor>>,
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut commands: Commands,
) {
    for contacts in collision_event_reader.read() {
        let entities = [contacts.0, contacts.1];
        let has_player = entities.iter().find(|&&e| player_query.get(e).is_ok());
        let has_attack = entities.iter().find(|&&e| attack_query.get(e).is_ok());

        if let (Some(&player_child), Some(&_)) = (has_player, has_attack) {
            if let Ok((_, parent)) = player_query.get(player_child) {
                commands
                    .entity(parent.get())
                    .insert((PlayerDamage(90.), PlayerRepulse(10., 0.)));
            }
        }
    }
}
