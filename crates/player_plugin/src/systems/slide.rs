use bevy::prelude::*;
use controll_plugin::Controllable;
use slide_system::{SlideDetector, Sliding, SlidingAllowed};

pub fn handle_slide(
    mut query: Query<
        (Entity, &Controllable),
        (With<SlidingAllowed>, Without<Sliding>, With<SlideDetector>),
    >,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (entity, controllable) in &mut query {
        if keys.pressed(controllable.down) {
            commands.entity(entity).insert(Sliding::default());
        }
    }
}
