use bevy::prelude::*;
use bullet_plugin::components::Bullet;
use components::PlaygroundItem;
use game_ui::components::{root_game_node, GameButton, GameRootContainer};

mod components;

use crate::GameState;

pub struct TestPlaygroundPlugin;

impl Plugin for TestPlaygroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::TestPlayGround), spawn_playground)
            .add_systems(OnExit(GameState::TestPlayGround), despawn_playground);
    }
}

fn spawn_playground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // BULLET
    let shape = Mesh2d(meshes.add(Rectangle::new(70., 10.)));
    let material = MeshMaterial2d(materials.add(Color::WHITE));

    commands.spawn((shape, material, Bullet));
    // MENU BUTTON
    commands
        .spawn((GameRootContainer, root_game_node(), PlaygroundItem))
        .with_children(|parent| {
            parent
                .spawn(GameButton::with_size_func(
                    Val::Px(50.),
                    Val::Px(120.),
                    "on_click_exit_game",
                ))
                .with_children(|parent| {
                    parent.spawn(Text::new("Exit"));
                });
        });
}

fn despawn_playground(mut commands: Commands, query: Query<Entity, With<PlaygroundItem>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
