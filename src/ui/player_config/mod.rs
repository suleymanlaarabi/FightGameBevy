use bevy::prelude::*;
use components::gamepad_root_container_node;
use game_ui::{
    builder::NodeBuilder,
    components::{root_game_node, GameButton, GameRootContainer},
};
use player_plugin::components::config::PlayersConfig;

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

fn spawn_ui(mut commands: Commands, players_config: Res<PlayersConfig>) {
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
            for (_, config) in players_config.0.iter().enumerate() {
                parent.spawn((
                    NodeBuilder::from_min_height(150.).build(),
                    GameButton::default(),
                    BorderRadius::all(Val::Px(10.)),
                    ImageNode::from_atlas_image(
                        config.image.clone(),
                        TextureAtlas {
                            index: config.preview as usize,
                            layout: config.atlas.clone(),
                            ..default()
                        },
                    ),
                ));
            }
        });
}
