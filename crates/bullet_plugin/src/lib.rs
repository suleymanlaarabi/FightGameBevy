use bevy::prelude::*;
use systems::bullet_collision_check;

pub mod components;
mod systems;

#[derive(Default)]
pub struct BulletPlugin<S: States> {
    pub need_states: Option<S>,
}

impl<S: States> Plugin for BulletPlugin<S> {
    fn build(&self, app: &mut App) {
        if let Some(state) = &self.need_states {
            app.add_systems(
                Update,
                bullet_collision_check.run_if(in_state(state.clone())),
            );
        } else {
            app.add_systems(Update, bullet_collision_check);
        }
    }
}
