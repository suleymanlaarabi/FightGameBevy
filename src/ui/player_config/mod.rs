use bevy::prelude::*;
use components::{gamepad_root_container_node, PlayerConfigApplyButton};
use config_plugin::resource_insertion::PlayersConfig;
use controll_plugin::ConnectedControll;
use game_ui::{
    builder::NodeBuilder,
    components::{root_game_node, GameButton, GameRootContainer, NORMAL_BUTTON},
};
use systems::on_player_config_select;

use crate::GameState;

mod components;
mod systems;

pub struct PlayerConfigUiPlugin;

impl Plugin for PlayerConfigUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            on_player_config_select.run_if(in_state(GameState::PlayerConfig)),
        )
        .add_systems(OnEnter(GameState::PlayerConfig), spawn_ui)
        .add_systems(OnExit(GameState::PlayerConfig), despawn_ui);
    }
}

fn despawn_ui(mut commands: Commands, query: Query<Entity, With<GameRootContainer>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_ui(
    mut commands: Commands,
    players_config: Res<PlayersConfig>,
    query: Query<Entity, With<ConnectedControll>>,
) {
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
            for (config_index, config) in players_config.0.iter().enumerate() {
                parent
                    .spawn((
                        NodeBuilder::from_min_height(180.)
                            .flex()
                            .justify_center()
                            .align_center()
                            .build(),
                        BackgroundColor(NORMAL_BUTTON),
                        BorderRadius::all(Val::Px(10.)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            ImageNode::from_atlas_image(
                                config.image.clone(),
                                TextureAtlas {
                                    index: config.preview as usize,
                                    layout: config.atlas.clone(),
                                    ..default()
                                },
                            ),
                            NodeBuilder::from_min_height(130.).build(),
                        ));
                        parent
                            .spawn(
                                NodeBuilder::from_flex_direction(FlexDirection::Row)
                                    .flex()
                                    .justify_center()
                                    .gap(5.)
                                    .padding_x(3.)
                                    .flex_direction(FlexDirection::Column)
                                    .build(),
                            )
                            .with_children(|parent| {
                                for (i, entity) in query.iter().enumerate() {
                                    parent
                                        .spawn((
                                            GameButton::default(),
                                            NodeBuilder::from_size(140., 30.)
                                                .padding_left(13.)
                                                .border_width(3.)
                                                .build(),
                                            PlayerConfigApplyButton(entity, config_index),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(Text::new(format!("Player: {}", i)));
                                        });
                                }
                            });
                    });
            }
        });
}
