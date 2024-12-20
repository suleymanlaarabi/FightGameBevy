use bevy::{color::palettes::tailwind::GREEN_200, prelude::*};
use game_ui::components::{game_button_node, NORMAL_BUTTON};

pub fn player_life_node() -> Node {
    Node {
        width: Val::Px(280.),
        height: Val::Px(40.),
        justify_content: JustifyContent::Start,
        ..game_button_node()
    }
}

#[derive(Component, Default)]
#[require(
    Node(player_life_node),
    BorderRadius(|| BorderRadius::all(Val::Px(10.))),
    BorderColor(|| BorderColor(Color::srgba(0.85, 0.85, 0.85, 0.7))),
    BackgroundColor(|| BackgroundColor(NORMAL_BUTTON)),
)]
pub struct PlayerLifeUi;

pub fn life_ui_progress_node() -> Node {
    Node {
        width: Val::Px(280.),
        left: Val::Px(-2.),
        ..player_life_node()
    }
}

#[derive(Component)]
#[require(
    Node(life_ui_progress_node),
    BorderRadius(|| BorderRadius::all(Val::Px(10.))),
    BackgroundColor(|| BackgroundColor(Color::Srgba(GREEN_200))),
)]
pub struct PlayerLifeUiProgress(pub Entity);

pub fn last_player_win_node() -> Node {
    Node {
        ..game_button_node()
    }
}

#[derive(Component)]
#[require(
    Node(last_player_win_node),
    BorderRadius(|| BorderRadius::all(Val::Px(10.))),
    BorderColor(|| BorderColor(Color::srgba(0.85, 0.85, 0.85, 0.7))),
    BackgroundColor(|| BackgroundColor(NORMAL_BUTTON)),
)]
pub struct LastPlayerWin;

// impl LastPlayerWin {
//     pub fn new(commands: &mut Commands) -> impl Bundle {
//         LastPlayerWin
//     }
// }
