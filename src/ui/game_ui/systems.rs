use bevy::prelude::*;
use player_plugin::components::PlayerLife;

use super::components::PlayerLifeUiProgress;

pub fn sync_player_life_and_ui(
    mut ui_life: Query<(&PlayerLifeUiProgress, &mut Node)>,
    query: Query<(Entity, &PlayerLife)>,
) {
    for (player_entity, player_life) in &query {
        for (player_ui, mut node) in &mut ui_life {
            if player_entity == player_ui.0 {
                node.width = Val::Px(player_life.0);
            }
        }
    }
}
