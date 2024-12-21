use attack_plugin::Attack;
use avian2d::prelude::Collider;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    player::components::{Controllable, Player},
    GameState,
};

use super::attack_collision::{detect_attack_collision, mark_attack};

pub struct PlayerAttackPlugin;

#[derive(Component, Default)]
pub struct AttackHitMarker(pub Option<f32>);

impl Plugin for PlayerAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (check_attack, detect_attack_collision).run_if(in_state(GameState::InFight)),
        )
        .add_systems(
            FixedUpdate,
            mark_attack.run_if(in_state(GameState::InFight)),
        );
    }
}

pub fn generate_attack(top: bool, entity: Entity, commands: &mut Commands) {
    if top {
        Attack::attack_with_collision(6, 0.5, Collider::rectangle(15., 10.), commands, entity);
    } else {
        Attack::attack_with_collision(
            thread_rng().gen_range(0..=4),
            0.5,
            Collider::rectangle(50., 10.),
            commands,
            entity,
        );
    }
}

pub fn check_attack(
    query: Query<(Entity, &Controllable), (With<Player>, Without<Attack>)>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (entity, controll) in &query {
        if keys.pressed(controll.hit) {
            generate_attack(keys.pressed(controll.top), entity, &mut commands);
        }
    }
}
