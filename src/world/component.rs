use std::fs::read_to_string;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MapCollision {
    pub position: Vec2,
    pub rect: Vec2,
}

#[derive(Deserialize, Serialize)]
pub struct MapRepresentation {
    pub image_path: String,
    pub collisions: Vec<MapCollision>,
}

impl MapRepresentation {
    pub fn from_file(path: impl Into<String>) -> Self {
        let map: Self =
            serde_json::from_str(&read_to_string(path.into()).expect("Unable to read the file"))
                .expect("Unable to parse the file");
        map
    }
}

#[derive(Component)]
pub struct MapParent;

#[derive(Component)]
pub struct WorldCollision;

#[derive(Component, Default)]
pub struct WorldSpawnRequest(pub usize);
