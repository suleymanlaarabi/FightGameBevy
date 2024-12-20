use bevy::prelude::*;

use crate::systems::flappy_button_interaction;

pub const NORMAL_BUTTON: Color = Color::srgba(0.65, 0.65, 0.65, 0.5);
pub const HOVERED_BUTTON: Color = Color::srgba(0.75, 0.75, 0.75, 0.6);

pub fn game_button_node() -> Node {
    Node {
        width: Val::Px(200.),
        height: Val::Px(70.),
        border: UiRect::all(Val::Px(2.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

#[derive(Component, Default)]
#[require(
    Button,
    Node(game_button_node),
    BorderRadius(|| BorderRadius::all(Val::Px(10.))),
    BorderColor(|| BorderColor(Color::srgba(0.85, 0.85, 0.85, 0.7))),
    BackgroundColor(|| BackgroundColor(NORMAL_BUTTON)),
)]
pub struct GameButton(pub Option<String>);

impl GameButton {
    pub fn with_size(height: Val, width: Val) -> impl Bundle {
        let mut node = game_button_node();
        node.height = height;
        node.width = width;
        (GameButton(None), node)
    }
    pub fn with_size_func(height: Val, width: Val, system: impl Into<String>) -> impl Bundle {
        let mut node = game_button_node();
        node.height = height;
        node.width = width;
        (GameButton(Some(system.into())), node)
    }
}

pub fn root_container_node() -> Node {
    Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(10.)),
        column_gap: Val::Px(10.),
        row_gap: Val::Px(10.),
        ..default()
    }
}

pub fn root_game_node() -> Node {
    Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        justify_content: JustifyContent::Center,
        padding: UiRect::all(Val::Px(10.)),
        column_gap: Val::Px(10.),
        ..default()
    }
}

#[derive(Component, Default)]
#[require(Node(root_container_node))]
pub struct GameRootContainer;

#[derive(Component)]
pub struct GameButtonClicked;

pub struct GameButtonPlugin;

impl Plugin for GameButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, flappy_button_interaction);
    }
}
