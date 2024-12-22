use attack_plugin::Attack;
use avian2d::prelude::Collider;
use bevy::prelude::*;
use controll_plugin::Controllable;
use player_plugin::components::Player;
use rand::{Rng, thread_rng};
pub mod attack_collision;
use attack_collision::detect_attack_collision;

pub struct PlayerAttackPlugin<S: States> {
    pub need_state: Option<S>,
}

impl<S: States> Plugin for PlayerAttackPlugin<S> {
    fn build(&self, app: &mut App) {
        let systems_update = (check_attack, detect_attack_collision);
        if let Some(state) = &self.need_state {
            app.add_systems(Update, systems_update.run_if(in_state(state.clone())));
        } else {
            app.add_systems(Update, (check_attack, detect_attack_collision));
        }
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
