use animation_plugin::CustomAnimationPlugin;
use attack_plugin::AttackPlugin;
use avian2d::{prelude::PhysicsDebugPlugin, PhysicsPlugins};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use grounded_plugin::CheckGroundPlugin;
use player::PlayerPlugin;
use resources::AppResourcePlugin;
use setup::GameSetupPlugin;
use slide_system::SlidePlugin;
use world::WorldPlugin;
mod player;
mod resources;
mod setup;
mod world;
use avian2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "game".to_string(),
                        resolution: Vec2::new(1280., 720.).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),
            //PhysicsDebugPlugin::default(),
            //WorldInspectorPlugin::default(),
            CustomAnimationPlugin,
            AttackPlugin,
            CheckGroundPlugin,
            AppResourcePlugin,
            PlayerPlugin,
            GameSetupPlugin,
            WorldPlugin,
            SlidePlugin,
        ))
        .insert_resource(Gravity::default())
        .run();
}
