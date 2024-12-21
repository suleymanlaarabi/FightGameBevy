use bevy::prelude::*;
use components::{gamepad_root_container_node, GamepadCard};
use game_ui::components::{root_game_node, GameButton, GameRootContainer};

use crate::GameState;

mod components;

pub struct PlayerConfigUiPlugin;

impl Plugin for PlayerConfigUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PlayerConfig), spawn_ui)
            .add_systems(OnExit(GameState::PlayerConfig), despawn_ui);
    }
}

fn despawn_ui(mut commands: Commands, query: Query<Entity, With<GameRootContainer>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_ui(mut commands: Commands, query: Query<&Gamepad>) {
    commands
        .spawn((GameRootContainer, root_game_node()))
        .with_children(|parent| {
            parent
                .spawn((GameButton::with_size_func(
                    Val::Px(50.),
                    Val::Px(170.),
                    "on_click_exit_game",
                ),))
                .with_children(|parent| {
                    parent.spawn(Text::new("Exit"));
                });
        });
    commands
        .spawn((GameRootContainer, gamepad_root_container_node()))
        .with_children(|parent| {
            for (i, _) in query.iter().enumerate() {
                parent.spawn(GamepadCard::new(i));
            }
            parent.spawn(GamepadCard::new(0));
        });
}