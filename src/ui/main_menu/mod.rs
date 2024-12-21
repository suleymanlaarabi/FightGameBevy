use bevy::prelude::*;
use game_ui::components::{GameButton, GameRootContainer};

use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartMenu), spawn_menu)
            .add_systems(OnExit(GameState::StartMenu), despawn_menu);
    }
}

#[derive(Component)]
struct PlayButton;

fn spawn_menu(mut commands: Commands) {
    commands.spawn(GameRootContainer).with_children(|parent| {
        parent
            .spawn((
                GameButton::with_size_func(Val::Px(50.), Val::Px(120.), "on_click_play"),
                PlayButton,
            ))
            .with_children(|parent| {
                parent.spawn(Text::new("Play"));
            });
        parent
            .spawn((GameButton::with_size_func(
                Val::Px(50.),
                Val::Px(120.),
                "on_click_settings_game",
            ),))
            .with_children(|parent| {
                parent.spawn(Text::new("Settings"));
            });
        parent
            .spawn((GameButton::with_size_func(
                Val::Px(50.),
                Val::Px(170.),
                "on_click_player_config",
            ),))
            .with_children(|parent| {
                parent.spawn(Text::new("PlayerConfig"));
            });
    });
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<GameRootContainer>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
