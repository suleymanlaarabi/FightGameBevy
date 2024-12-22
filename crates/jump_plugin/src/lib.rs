use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Jump(pub f32);

pub struct JumpPlugin;

impl Plugin for JumpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_jump);
    }
}

pub fn apply_jump(mut query: Query<(Entity, &mut LinearVelocity, &Jump)>, mut commands: Commands) {
    for (entity, mut velocity, jump) in &mut query {
        velocity.y = jump.0;
        commands.entity(entity).remove::<Jump>();
    }
}
