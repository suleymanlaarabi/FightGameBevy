use attack_plugin::Attack;
use avian2d::prelude::LinearVelocity;
use bevy::{
    input::gamepad::{GamepadConnection, GamepadEvent},
    prelude::*,
};
use grounded_plugin::IsGrounded;
use rand::{thread_rng, Rng};
use slide_system::{Sliding, SlidingAllowed};

use crate::{
    player::{
        components::{Player, PlayerLife},
        systems::{
            attack::generate_attack, collision_detector, PLAYER_JUMP_FORCE, PLAYER_VELOCITY_X,
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
        app.add_systems(OnEnter(GameState::InFight), spawn_first_gamepad_player)
            .add_systems(FixedUpdate, handle_gamepad_connections)
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
                    velocity.y = PLAYER_JUMP_FORCE;
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

fn spawn_first_gamepad_player(
    mut commands: Commands,
    query: Query<Entity, With<Gamepad>>,
    image: Res<PlayerTileSheet>,
    atlas_layout: Res<AdventurerAtlasLayout>,
) {
    for entity in &query {
        let mut rng = thread_rng();
        let random_number = rng.gen_range(-250..=250);
        commands
            .spawn((
                Player::no_control(&image, &atlas_layout, random_number as f32),
                PlayerLife(290.),
                GamePadControlled(entity),
            ))
            .with_children(|parent| {
                parent.spawn(collision_detector());
            });
    }
}

fn handle_gamepad_connections(
    mut evr_gamepad: EventReader<GamepadEvent>,
    player_query: Query<(Entity, &GamePadControlled)>,
    mut commands: Commands,
    image: Res<PlayerTileSheet>,
    atlas_layout: Res<AdventurerAtlasLayout>,
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
                commands
                    .spawn((
                        Player::no_control(&image, &atlas_layout, 200.),
                        PlayerLife(290.),
                        GamePadControlled(ev_conn.gamepad),
                    ))
                    .with_children(|parent| {
                        parent.spawn(collision_detector());
                    });
            }
            GamepadConnection::Disconnected => {
                for controlled_player in &player_query {
                    if controlled_player.1 .0 == ev_conn.gamepad {
                        commands.entity(controlled_player.0).despawn_recursive();
                    }
                }
            }
        }
    }
}
