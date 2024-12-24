use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

use bevy::prelude::*;
use gun_plugin::components::Gun;
use serde::{Deserialize, Serialize};

// JSON REPR
#[derive(Deserialize, Serialize)]
pub struct GunConfig {
    pub name: String,
    pub image_path: Option<String>,
    pub frequency: f32,
}

#[derive(Resource)]
pub struct GunConfigs(pub Vec<Gun>);

impl FromWorld for GunConfigs {
    fn from_world(world: &mut World) -> Self {
        let mut guns_config: Vec<Gun> = Vec::new();
        let dir = read_dir("assets/guns").expect("Guns dir not found");
        for entry in dir {
            let entry = entry.expect("error file in guns dir");
            let path = entry.path();
            let config = GunConfig::from_file(path);
            let server = world.resource_mut::<AssetServer>();
            let mut gun = Gun {
                frequency: config.frequency,
                image: None,
            };
            if let Some(image_path) = config.image_path {
                gun.image = Some(server.load(image_path));
            }
            guns_config.push(gun);
        }
        Self(guns_config)
    }
}

impl GunConfig {
    pub fn from_file(path: PathBuf) -> Self {
        serde_json::from_str(&read_to_string(path).expect("Unable to read gun file"))
            .expect("unable to parse gun file")
    }
}
