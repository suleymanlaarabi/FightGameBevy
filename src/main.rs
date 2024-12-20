use animation_plugin::CustomAnimationPlugin;
use attack_plugin::AttackPlugin;
use avian2d::PhysicsPlugins;
use bevy::prelude::*;
mod ui;
use game_ui::GameUiPlugin;
use gamepad::GamePadPlayerPlugin;
use grounded_plugin::CheckGroundPlugin;
use jump_plugin::JumpPlugin;
use player::PlayerPlugin;
use resources::AppResourcePlugin;
use setup::GameSetupPlugin;
use slide_system::SlidePlugin;
use ui::{settings::SettingsUiPlugin, CustomUiPlugin};
use world::WorldPlugin;
mod player;
mod resources;
mod setup;
mod world;
use avian2d::prelude::*;
mod gamepad;
mod scene;
mod wrapper;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    StartMenu,
    InFight,
    Settings,
}

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
            CustomAnimationPlugin,
            AttackPlugin,
            CheckGroundPlugin,
            AppResourcePlugin,
            PlayerPlugin,
            GameSetupPlugin,
            WorldPlugin,
            SlidePlugin,
            GameUiPlugin,
            CustomUiPlugin,
            SettingsUiPlugin,
            GamePadPlayerPlugin,
        ))
        .add_plugins(JumpPlugin)
        .insert_state(GameState::StartMenu)
        .insert_resource(Gravity::default())
        .run();
}
