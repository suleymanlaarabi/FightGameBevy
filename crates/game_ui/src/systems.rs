use bevy::prelude::*;

use crate::{
    ButtonMapSystems,
    components::{GameButton, GameButtonClicked, HOVERED_BUTTON, NORMAL_BUTTON},
};

pub fn flappy_button_interaction(
    mut interaction_query: Query<
        (Entity, &GameButton, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    map: Res<ButtonMapSystems>,
    mut commands: Commands,
) {
    for (entity, button, interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                background_color.0 = HOVERED_BUTTON;
            }
            Interaction::Pressed => {
                commands.entity(entity).insert(GameButtonClicked);
                if let Some(system) = button.0.clone() {
                    let id = map.0.get(&system).expect("System not found");
                    commands.run_system(id.clone());
                }
            }
            _ => {
                background_color.0 = NORMAL_BUTTON;
            }
        }
    }
}
