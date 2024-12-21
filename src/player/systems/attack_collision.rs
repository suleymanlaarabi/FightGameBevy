use attack_plugin::AttackSensor;
use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    player::components::{Player, PlayerLife},
    ui::game_ui::components::PlayerLifeUiProgress,
};

use super::attack::AttackHitMarker;

pub fn mark_attack(
    mut query: Query<(Entity, &mut PlayerLife, &AttackHitMarker)>,
    mut ui_life: Query<(&PlayerLifeUiProgress, &mut Node)>,
    mut commands: Commands,
) {
    for (entity, mut player_life, marker) in &mut query {
        if let Some(degat) = marker.0 {
            player_life.0 -= degat;
        } else {
            player_life.0 -= 30.;
        }
        if player_life.0 <= 0. {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        for (life, mut node) in &mut ui_life {
            if life.0 == entity {
                node.width = Val::Px(player_life.0);
            }
        }
        commands.entity(entity).remove::<AttackHitMarker>();
    }
}

pub fn detect_attack_collision(
    player_query: Query<Entity, With<Player>>,
    attack_query: Query<Entity, With<AttackSensor>>,
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut commands: Commands,
) {
    for contacts in collision_event_reader.read() {
        let entities = [contacts.0, contacts.1];
        let has_player = entities.iter().find(|&&e| player_query.get(e).is_ok());
        let has_attack = entities.iter().find(|&&e| attack_query.get(e).is_ok());

        if let (Some(&player), Some(_attack)) = (has_player, has_attack) {
            commands.entity(player).insert(AttackHitMarker::default());
        }
    }
}
