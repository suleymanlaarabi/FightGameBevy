use bevy::prelude::*;
use components::{PlayerLifeUi, PlayerLifeUiProgress};
use game_ui::components::{root_game_node, GameButton, GameRootContainer};
use player_plugin::components::{Player, PlayerLife};
use systems::sync_player_life_and_ui;

pub mod components;
mod systems;

use crate::GameState;

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InFight),
            (spawn_menu, spawn_life_container),
        )
        .add_systems(OnExit(GameState::InFight), (despawn_menu, despawn_life))
        .add_systems(
            Update,
            (detect_removals, spawn_life.after(spawn_life_container))
                .run_if(in_state(GameState::InFight)),
        )
        .add_systems(
            FixedUpdate,
            sync_player_life_and_ui.run_if(in_state(GameState::InFight)),
        );
    }
}

#[derive(Component)]
struct PlayerLifeUiContainer;

fn spawn_life_container(mut commands: Commands) {
    commands.spawn((
        Node {
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexEnd,
            ..root_game_node()
        },
        PlayerLifeUiContainer,
    ));
}

fn detect_removals(
    mut removals: RemovedComponents<Player>,
    query: Query<(&PlayerLifeUiProgress, &Parent)>,
    mut commands: Commands,
) {
    for entity in removals.read() {
        for (life_ui, parent) in &query {
            if entity == life_ui.0 {
                commands.entity(parent.get()).despawn_recursive();
            }
        }
    }
}

fn spawn_life(
    mut commands: Commands,
    query_container: Query<Entity, With<PlayerLifeUiContainer>>,
    life_query: Query<Entity, Added<PlayerLife>>,
) {
    let entity = query_container.single();
    for player_entity in &life_query {
        commands.entity(entity).with_children(|parent| {
            parent.spawn(PlayerLifeUi).with_children(|parent| {
                parent.spawn(PlayerLifeUiProgress(player_entity));
            });
        });
    }
}

fn despawn_life(mut commands: Commands, query: Query<Entity, With<PlayerLifeUiContainer>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_menu(mut commands: Commands) {
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
            parent
                .spawn(GameButton::with_size_func(
                    Val::Px(50.),
                    Val::Px(120.),
                    "refresh_maps",
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Refresh"));
                });
        });
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<GameRootContainer>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
