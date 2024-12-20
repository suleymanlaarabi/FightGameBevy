use avian2d::prelude::*;
use bevy::prelude::*;

pub struct CheckGroundPlugin;

#[derive(Component, Default)]
pub struct GroundDetector;

#[derive(Component)]
pub struct IsGrounded;

impl Plugin for CheckGroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_ground, check_unground));
    }
}

fn check_ground(
    query: Query<(Entity, &Parent), (With<GroundDetector>, Without<IsGrounded>)>,
    mut commands: Commands,
    mut collision_event_reader: EventReader<Collision>,
) {
    for contacts in collision_event_reader.read() {
        for (child_entity, parent) in &query {
            if child_entity == contacts.0.entity1 || child_entity == contacts.0.entity2 {
                let parent_entity = parent.get();
                commands.entity(parent_entity).insert(IsGrounded);
            }
        }
    }
}

fn check_unground(
    query: Query<(Entity, &Parent), With<GroundDetector>>,
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionEnded>,
) {
    for contacts in collision_event_reader.read() {
        for (child_entity, parent) in &query {
            if child_entity == contacts.0 || child_entity == contacts.1 {
                let parent_entity = parent.get();
                commands.entity(parent_entity).remove::<IsGrounded>();
            }
        }
    }
}
