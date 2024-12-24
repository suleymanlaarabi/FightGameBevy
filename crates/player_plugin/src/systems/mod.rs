pub mod damage;
pub mod fail;
pub mod movement;
pub mod repulse;
pub mod slide;

use avian2d::prelude::*;
use bevy::prelude::*;

use config_plugin::{player::PlayerConfig, player::PlayersConfig};
use controll_plugin::{ConnectedControll, GamepadControlled};
use slide_system::Sliding;

use crate::components::{Player, spawn_player};

pub fn boost_velocity(mut query: Query<&mut LinearVelocity, With<Sliding>>) {
    for mut velocity in &mut query {
        velocity.x *= 1.5;
    }
}

pub fn despawn_player(query: Query<Entity, With<Player>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_player_system(
    mut commands: Commands,
    players: Query<(&ConnectedControll, Option<&PlayerConfig>)>,
    players_config: Res<PlayersConfig>,
) {
    let config = players_config.0.get(1).expect("No player config found");

    for (player, player_config) in &players {
        match player {
            ConnectedControll::Keyboard(controll) => {
                if let Some(config) = player_config {
                    spawn_player(&mut commands, &config, controll.clone());
                } else {
                    spawn_player(&mut commands, &config, controll.clone());
                }
            }
            ConnectedControll::Gamepad(entity) => {
                if let Some(config) = player_config {
                    spawn_player(&mut commands, &config, GamepadControlled(entity.clone()));
                } else {
                    spawn_player(&mut commands, &config, GamepadControlled(entity.clone()));
                }
            }
        }
    }
}
