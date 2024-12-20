use bevy::prelude::*;
use game_ui::components::{game_button_node, root_container_node, GameButton, GameRootContainer};

fn map_card_node() -> Node {
    Node {
        height: Val::Px(160.),
        width: Val::Px(240.),
        ..game_button_node()
    }
}

#[derive(Component)]
#[require(GameButton)]
pub struct MapCard;

fn map_root_container_node() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        flex_wrap: FlexWrap::Wrap,
        row_gap: Val::Px(0.),
        ..root_container_node()
    }
}

#[derive(Component)]
#[require(GameRootContainer)]
pub struct MapContainer;

impl MapContainer {
    pub fn new() -> impl Bundle {
        (GameRootContainer, map_root_container_node())
    }
}

impl MapCard {
    pub fn new() -> impl Bundle {
        (MapCard, map_card_node())
    }
}