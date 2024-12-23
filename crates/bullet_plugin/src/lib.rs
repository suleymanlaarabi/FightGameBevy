use bevy::prelude::*;
use components::BulletHitEvent;
use systems::{bullet_collision_check, bullet_out_screen_check};

pub mod components;
mod systems;

#[derive(Default)]
pub struct BulletPlugin<S: States> {
    pub need_states: Option<S>,
}

impl<S: States> Plugin for BulletPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletHitEvent>();
        if let Some(state) = &self.need_states {
            app.add_systems(
                Update,
                (bullet_collision_check, bullet_out_screen_check).run_if(in_state(state.clone())),
            );
        } else {
            app.add_systems(Update, (bullet_collision_check, bullet_out_screen_check));
        }
    }
}
