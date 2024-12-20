use bevy::prelude::*;
use slide_system::{Sliding, SlidingAllowed};

use crate::player::components::Controllable;

pub fn handle_slide(
    mut query: Query<(Entity, &Controllable), (With<SlidingAllowed>, Without<Sliding>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (entity, controllable) in &mut query {
        if keys.pressed(controllable.down) {
            commands.entity(entity).insert(Sliding::default());
        }
    }
}
