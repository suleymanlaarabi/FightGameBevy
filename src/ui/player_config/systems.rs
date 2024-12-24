use bevy::prelude::*;
use config_plugin::player::PlayersConfig;

use super::components::PlayerConfigApplyButton;
use game_ui::components::GameButtonClicked;

pub fn on_player_config_select(
    query: Query<(Entity, &PlayerConfigApplyButton), Added<GameButtonClicked>>,
    players_config: Res<PlayersConfig>,
    mut commands: Commands,
) {
    for (entity, config) in &query {
        commands.entity(entity).remove::<GameButtonClicked>();
        commands
            .entity(config.0)
            .insert(players_config.0[config.1].clone());
    }
}
