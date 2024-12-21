use attack_plugin::Attack;
use avian2d::prelude::LinearVelocity;
use bevy::{
    input::gamepad::{GamepadConnection, GamepadEvent},
    prelude::*,
};
use grounded_plugin::IsGrounded;
use jump_plugin::Jump;
use rand::{thread_rng, Rng};
use slide_system::{Sliding, SlidingAllowed};

use crate::{
    player::{
        components::{ConnectedPlayer, Player},
        systems::{
            attack::generate_attack,
            collision_detector,
            movement::{PLAYER_JUMP_FORCE, PLAYER_VELOCITY_X},
        },
    },
    resources::{AdventurerAtlasLayout, PlayerTileSheet},
    GameState,
};

pub struct GamePadPlayerPlugin;

#[derive(Component)]
pub struct GamePadControlled(pub Entity);

impl Plugin for GamePadPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, handle_gamepad_connections)
            .add_systems(Update, handle_gamepad_input);
    }
}

fn handle_gamepad_input(
    gamepads: Query<(Entity, &Gamepad)>,
    mut player_query: Query<(
        Entity,
        &mut LinearVelocity,
        &GamePadControlled,
        Option<&Attack>,
        Option<&IsGrounded>,
        Option<&SlidingAllowed>,
    )>,
    mut commands: Commands,
) {
    for (player_entity, mut velocity, gamepad_controll, attack, grounded, sliding_allowed) in
        &mut player_query
    {
        velocity.x = 0.;
        for (entity, gamepad) in &gamepads {
            if entity == gamepad_controll.0 {
                if gamepad.pressed(GamepadButton::DPadRight) {
                    velocity.x = PLAYER_VELOCITY_X;
                } else if gamepad.pressed(GamepadButton::DPadLeft) {
                    velocity.x -= PLAYER_VELOCITY_X;
                }
                if grounded.is_some() && gamepad.just_pressed(GamepadButton::South) {
                    commands
                        .entity(player_entity)
                        .insert(Jump(PLAYER_JUMP_FORCE));
                }
                if sliding_allowed.is_some() && gamepad.just_pressed(GamepadButton::West) {
                    commands.entity(player_entity).insert(Sliding::default());
                }
                if attack.is_none() && gamepad.just_pressed(GamepadButton::East) {
                    generate_attack(
                        gamepad.pressed(GamepadButton::DPadUp),
                        player_entity,
                        &mut commands,
                    );
                }
            }
        }
    }
}

fn handle_gamepad_connections(
    mut evr_gamepad: EventReader<GamepadEvent>,
    player_query: Query<(Entity, &GamePadControlled)>,
    connected_query: Query<(Entity, &ConnectedPlayer)>,
    mut commands: Commands,
) {
    for evt in evr_gamepad.read() {
        let GamepadEvent::Connection(ev_conn) = evt else {
            continue;
        };
        match &ev_conn.connection {
            GamepadConnection::Connected {
                name: _,
                vendor_id: _,
                product_id: _,
            } => {
                commands.spawn(ConnectedPlayer::Gamepad(ev_conn.gamepad));
            }
            GamepadConnection::Disconnected => {
                for controlled_player in &player_query {
                    if controlled_player.1 .0 == ev_conn.gamepad {
                        commands.entity(controlled_player.0).despawn_recursive();
                    }
                }
                for (connected_player_entity, connected_player) in &connected_query {
                    match connected_player {
                        ConnectedPlayer::Gamepad(entity) => {
                            if entity == &ev_conn.gamepad {
                                commands.entity(connected_player_entity).despawn();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
