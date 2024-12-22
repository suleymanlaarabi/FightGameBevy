use animation::handle_animation;
use bevy::prelude::*;

mod animation;

pub struct PlayerAnimationPlugin<S: States> {
    pub need_state: Option<S>,
}

impl<S: States> Plugin for PlayerAnimationPlugin<S> {
    fn build(&self, app: &mut App) {
        if let Some(state) = &self.need_state {
            app.add_systems(Update, handle_animation.run_if(in_state(state.clone())));
        } else {
            app.add_systems(Update, handle_animation);
        }
    }
}
