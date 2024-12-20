use bevy::prelude::*;
use components::{MapCard, MapContainer};
use game_ui::components::{root_game_node, GameButton, GameRootContainer};

mod components;

use crate::GameState;

pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), spawn_ui)
            .add_systems(OnExit(GameState::Settings), despawn_ui);
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn(MapContainer::new()).with_children(|parent| {
        for _ in 0..4 {
            parent.spawn(MapCard::new());
        }
    });
    commands
        .spawn((GameRootContainer, root_game_node()))
        .with_children(|parent| {
            parent
                .spawn(GameButton::with_size_func(
                    Val::Px(50.),
                    Val::Px(120.),
                    "on_click_exit_game",
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Exit"));
                });
        });
}

fn despawn_ui(
    mut commands: Commands,
    query: Query<Entity, With<MapContainer>>,
    quer2: Query<Entity, With<GameRootContainer>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &quer2 {
        commands.entity(entity).despawn_recursive();
    }
}
