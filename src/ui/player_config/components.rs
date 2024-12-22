use bevy::prelude::*;
use game_ui::components::root_container_node;

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
