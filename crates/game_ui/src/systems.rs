use bevy::{
    prelude::*,
    window::{PrimaryWindow, SystemCursorIcon},
    winit::cursor::CursorIcon,
};

use crate::{
    ButtonMapSystems,
    components::{GameButton, GameButtonClicked, HOVERED_BUTTON, NORMAL_BUTTON},
};

const CURSOR_POINTER: CursorIcon = CursorIcon::System(SystemCursorIcon::Pointer);
const DEFAULT_CURSOR: CursorIcon = CursorIcon::System(SystemCursorIcon::Default);

pub fn game_button_interaction(
    mut interaction_query: Query<
        (Entity, &GameButton, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    q_windows: Query<Entity, With<PrimaryWindow>>,
    map: Res<ButtonMapSystems>,
    mut commands: Commands,
) {
    for primary_window in &q_windows {
        for (entity, button, interaction, mut background_color) in &mut interaction_query {
            match *interaction {
                Interaction::Hovered => {
                    background_color.0 = HOVERED_BUTTON;
                    commands.entity(primary_window).insert(CURSOR_POINTER);
                }
                Interaction::Pressed => {
                    commands.entity(entity).insert(GameButtonClicked);
                    if let Some(system) = button.0.clone() {
                        let id = map.0.get(&system).expect("System not found");
                        commands.run_system(id.clone());
                    }
                }
                _ => {
                    commands.entity(primary_window).insert(DEFAULT_CURSOR);
                    background_color.0 = NORMAL_BUTTON;
                }
            }
        }
    }
}
