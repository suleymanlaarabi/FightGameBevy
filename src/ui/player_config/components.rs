use bevy::prelude::*;
use game_ui::components::{game_button_node, root_container_node, GameButton};

pub fn gamepad_root_container_node() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        flex_wrap: FlexWrap::Wrap,
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::FlexEnd,
        row_gap: Val::Px(0.),
        ..root_container_node()
    }
}

fn gamepad_card_node() -> Node {
    Node {
        height: Val::Px(160.),
        width: Val::Px(210.),
        padding: UiRect::all(Val::Px(13.)),
        ..game_button_node()
    }
}

#[derive(Component)]
#[require(GameButton)]
pub struct GamepadCard(pub usize);

impl GamepadCard {
    pub fn new(id: usize) -> impl Bundle {
        (GamepadCard(id), gamepad_card_node())
    }
}
