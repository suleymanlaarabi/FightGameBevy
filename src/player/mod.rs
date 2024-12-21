use bevy::prelude::*;
use components::{ConnectedPlayer, Controllable};
use resource::PlayerResourcePlugin;
use systems::{
    animation::PlayerAnimationPlugin,
    attack::PlayerAttackPlugin,
    boost_velocity, despawn_player,
    fail::handle_fail,
    movement::{handle_jump, move_player},
    slide::handle_slide,
    spawn_player,
};

use crate::GameState;

pub mod components;
pub mod player_config;
pub mod resource;
mod setup;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerAttackPlugin,
            PlayerAnimationPlugin,
            PlayerResourcePlugin,
        ))
        .add_systems(Startup, spanw_connected_player)
        .add_systems(OnEnter(GameState::InFight), spawn_player)
        .add_systems(OnExit(GameState::InFight), despawn_player)
        .add_systems(
            Update,
            (
                move_player.before(boost_velocity),
                handle_jump,
                handle_slide,
                handle_fail,
                boost_velocity,
            )
                .run_if(in_state(GameState::InFight)),
        );
    }
}

fn spanw_connected_player(mut commands: Commands) {
    commands.spawn(ConnectedPlayer::Keyboard(Controllable::default()));
}
