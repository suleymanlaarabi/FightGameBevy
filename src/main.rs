use animation_plugin::CustomAnimationPlugin;
use attack_plugin::AttackPlugin;
use bevy::prelude::*;
mod ui;
use bullet_plugin::BulletPlugin;
use game_ui::GameUiPlugin;
use grounded_plugin::CheckGroundPlugin;
use gun_plugin::GameGunPlugin;
use jump_plugin::JumpPlugin;
use one_shot_system::register_one_shot_system;
use player_animation_plugin::PlayerAnimationPlugin;
use player_attack_plugin::PlayerAttackPlugin;
use player_gamepad_plugin::GamePadPlayerPlugin;
use player_gun_plugin::PlayerGunPlugin;
use player_plugin::PlayerPlugin;
use setup::GameSetupPlugin;
use slide_system::SlidePlugin;
use test_playground::TestPlaygroundPlugin;
use ui::{player_config::PlayerConfigUiPlugin, settings::SettingsUiPlugin, CustomUiPlugin};
use world::WorldPlugin;
mod setup;
mod world;
use avian2d::prelude::*;
mod one_shot_system;
mod scene;
mod test_playground;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    StartMenu,
    InFight,
    Settings,
    PlayerConfig,
    TestPlayGround,
}

fn main() {
    let mut app = App::new();
    register_one_shot_system(&mut app);
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "game".to_string(),
                    resolution: Vec2::new(1280., 720.).into(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        PhysicsPlugins::default(),
        PhysicsDebugPlugin::default(),
        TestPlaygroundPlugin,
        CustomAnimationPlugin,
        AttackPlugin,
        CheckGroundPlugin,
        GameSetupPlugin,
        WorldPlugin,
        SlidePlugin,
        GameUiPlugin,
        CustomUiPlugin,
        SettingsUiPlugin,
        GamePadPlayerPlugin,
    ))
    .add_plugins((
        JumpPlugin,
        PlayerConfigUiPlugin,
        PlayerPlugin {
            state: GameState::InFight,
        },
        PlayerAnimationPlugin {
            need_state: Some(GameState::InFight),
        },
        PlayerAttackPlugin {
            need_state: Some(GameState::InFight),
        },
    ))
    .add_plugins((
        BulletPlugin::<GameState>::default(),
        GameGunPlugin,
        PlayerGunPlugin,
    ))
    .insert_state(GameState::StartMenu)
    .run();
}
