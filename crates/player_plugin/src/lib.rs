pub mod components;
pub mod resource;
pub mod systems;

use bevy::prelude::*;
use controll_plugin::{ConnectedControll, Controllable};
use resource::PlayerResourcePlugin;
use systems::{
    boost_velocity,
    damage::apply_damage,
    despawn_player,
    fail::handle_fail,
    movement::{handle_jump, move_player},
    repulse::apply_player_repulse,
    slide::handle_slide,
    spawn_player_system,
};

pub struct PlayerPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for PlayerPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerResourcePlugin)
            .add_systems(Startup, spanw_connected_player)
            .add_systems(OnEnter(self.state.clone()), spawn_player_system)
            .add_systems(OnExit(self.state.clone()), despawn_player)
            .add_systems(
                FixedUpdate,
                (apply_damage, apply_player_repulse).run_if(in_state(self.state.clone())),
            )
            .add_systems(
                Update,
                (
                    move_player.before(boost_velocity),
                    handle_jump,
                    handle_slide,
                    handle_fail,
                    boost_velocity,
                )
                    .run_if(in_state(self.state.clone())),
            );
    }
}

fn spanw_connected_player(mut commands: Commands) {
    commands.spawn(ConnectedControll::Keyboard(Controllable::default()));
}
