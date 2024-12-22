use bevy::prelude::*;
use grounded_plugin::IsGrounded;

pub struct SlidePlugin;

#[derive(Component, Deref, DerefMut)]
pub struct Sliding(Timer);

#[derive(Component, Deref, DerefMut)]
struct LastSlideTime(Timer);

#[derive(Component)]
pub struct SlidingAllowed;

#[derive(Component)]
pub struct SlideDetector;

impl Default for LastSlideTime {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Once))
    }
}

impl Default for Sliding {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Once))
    }
}

impl Plugin for SlidePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                on_sliding_insert,
                check_ground_exit,
                check_ground_enter,
                during_slide,
                during_not_slide,
            ),
        );
    }
}

fn on_sliding_insert(mut query: Query<Entity, Added<Sliding>>, mut commands: Commands) {
    for entity in &mut query {
        commands.entity(entity).remove::<SlidingAllowed>();
    }
}

fn during_not_slide(
    mut query: Query<(Entity, &mut LastSlideTime)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let delta = time.delta();
    for (entity, mut time) in &mut query {
        time.tick(delta);
        if time.just_finished() {
            commands.entity(entity).insert(SlidingAllowed);
            commands.entity(entity).remove::<LastSlideTime>();
        }
    }
}

fn during_slide(mut query: Query<(Entity, &mut Sliding)>, mut commands: Commands, time: Res<Time>) {
    let delta = time.delta();

    for (entity, mut sliding) in &mut query {
        sliding.tick(delta);
        if sliding.just_finished() {
            commands.entity(entity).remove::<Sliding>();
            commands.entity(entity).insert(LastSlideTime::default());
        }
    }
}

fn check_ground_exit(
    query: Query<
        Entity,
        (
            With<SlidingAllowed>,
            Without<IsGrounded>,
            With<SlideDetector>,
        ),
    >,
    mut commands: Commands,
) {
    for entity in &query {
        commands.entity(entity).remove::<SlidingAllowed>();
        commands.entity(entity).insert(LastSlideTime::default());
    }
}

fn check_ground_enter(
    mut query: Query<
        (Entity, &LastSlideTime),
        (
            Without<SlidingAllowed>,
            With<IsGrounded>,
            With<SlideDetector>,
        ),
    >,
    mut commands: Commands,
) {
    for (entity, time) in &mut query {
        if time.just_finished() {
            commands.entity(entity).insert(SlidingAllowed);
        }
    }
}
