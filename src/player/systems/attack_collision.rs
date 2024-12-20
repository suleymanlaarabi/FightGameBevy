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
            return;
        }
        for (life, mut node) in &mut ui_life {
            if life.0 == entity {
                node.width = Val::Px(player_life.0);
            }
        }
        commands.entity(entity).remove::<AttackHitMarker>();
    }
}

fn check_contact(contacts: &CollisionStarted, entity1: Entity, entity2: Entity) -> bool {
    if (contacts.0 == entity1 || contacts.1 == entity1)
        && (contacts.0 == entity2 || contacts.1 == entity2)
    {
        return true;
    }
    return false;
}

pub fn detect_attack_collision(
    player_query: Query<Entity, With<Player>>,
    attack_query: Query<Entity, With<AttackSensor>>,
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut commands: Commands,
) {
    for contacts in collision_event_reader.read() {
        for player_entity in &player_query {
            for attack_entity in &attack_query {
                if check_contact(contacts, player_entity, attack_entity) {
                    commands
                        .entity(player_entity)
                        .insert(AttackHitMarker::default());
                }
            }
        }
    }
}
