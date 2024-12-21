use bevy::prelude::*;
use serde::Deserialize;

pub struct GameConfigPlugin;

#[derive(Resource)]
pub struct ConfigJsonResource<T>(T)
where
    T: for<'de> Deserialize<'de>;

// impl<T: for<'de> Deserialize<'de>> ConfigJsonResource<T> {
//     fn from_file(world: &mut World, file_path: impl Into<String>) -> Self {
//         let content = read_to_string(file_path.into()).expect("Unable to read the file");
//         Self(serde_json::from_str(&content).expect("Unable to parse the file"))
//     }
// }

// impl Plugin for GameConfigPlugin {
//     fn build(&self, app: &mut App) {}
// }
