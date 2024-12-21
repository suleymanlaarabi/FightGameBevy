pub mod animation;
pub mod attack;
pub mod attack_collision;
pub mod fail;
pub mod movement;
pub mod slide;

use avian2d::prelude::*;
use bevy::prelude::*;
use slide_system::Sliding;

use crate::{gamepad::GamePadControlled, utils::generate_x_position};

use super::{
    components::{ConnectedPlayer, Player},
    player_config::PlayersConfig,
    setup::collision_detector,
};

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

pub fn spawn_player(
    mut commands: Commands,
    players: Query<&ConnectedPlayer>,
    players_config: Res<PlayersConfig>,
) {
    let config = players_config.0.get(1).expect("No player config found");
    for player in &players {
        match player {
            ConnectedPlayer::Keyboard(controll) => {
                commands
                    .spawn((
                        Player::full(&config.image, &config.atlas, -250., controll.clone()),
                        config.animation_config.clone(),
                    ))
                    .with_children(|parent| {
                        parent.spawn(collision_detector(config.ground_x));
                        parent.spawn((
                            Collider::capsule(config.collider_size.x, config.collider_size.y),
                            Transform::from_xyz(
                                config.collider_offset.x,
                                config.collider_offset.y,
                                0.,
                            ),
                            GlobalTransform::default(),
                        ));
                    });
            }
            ConnectedPlayer::Gamepad(entity) => {
                commands
                    .spawn((
                        Player::full(
                            &config.image,
                            &config.atlas,
                            generate_x_position(),
                            GamePadControlled(entity.clone()),
                        ),
                        config.animation_config.clone(),
                    ))
                    .with_children(|parent| {
                        parent.spawn(collision_detector(config.ground_x));
                        parent.spawn((
                            Collider::capsule(config.collider_size.x, config.collider_size.y),
                            Transform::from_xyz(
                                config.collider_offset.x,
                                config.collider_offset.y,
                                0.,
                            ),
                            GlobalTransform::default(),
                        ));
                    });
            }
        }
    }
}
