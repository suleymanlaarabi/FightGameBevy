use bevy::prelude::*;
use components::{MapCard, MapContainer};
use game_ui::components::{root_game_node, GameButton, GameButtonClicked, GameRootContainer};

mod components;

use crate::{
    world::{
        component::{SelectedWorld, WorldSpawnRequest},
        WorldMaps,
    },
    GameState,
};

pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, on_clicked)
            .add_systems(OnEnter(GameState::Settings), spawn_ui)
            .add_systems(OnExit(GameState::Settings), despawn_ui);
    }
}

fn on_clicked(
    query: Query<(Entity, &MapCard), With<GameButtonClicked>>,
    mut commands: Commands,
    mut selected_world: ResMut<SelectedWorld>,
) {
    let mut spawned = false;
    for (entity, map) in &query {
        if !spawned {
            commands.spawn(WorldSpawnRequest(map.0 as isize));
            spawned = true;
            selected_world.0 = map.0;
        }
        commands.entity(entity).remove::<GameButtonClicked>();
    }
}

fn spawn_ui(mut commands: Commands, maps: Res<WorldMaps>) {
    commands.spawn(MapContainer::new()).with_children(|parent| {
        for (i, map) in maps.0.iter().enumerate() {
            parent.spawn(MapCard::new(i)).with_children(|parent| {
                parent.spawn((
                    Node {
                        height: Val::Px(140.),
                        width: Val::Px(220.),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(10.)),
                    ImageNode::new(map.0.clone()),
                ));
            });
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
