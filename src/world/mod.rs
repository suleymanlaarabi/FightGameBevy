use std::fs::read_dir;

use bevy::prelude::*;
use component::{MapCollision, MapRepresentation, SelectedWorld};
use systems::{spawn_world, world_spawn_handle};

pub mod component;
mod systems;

pub struct WorldPlugin;

pub type WorldMap = (
    Handle<Image>,
    Option<Handle<AudioSource>>,
    Vec<MapCollision>,
);

#[derive(Deref, DerefMut, Resource, TypePath, Asset)]
pub struct WorldMaps(pub Vec<WorldMap>);

impl FromWorld for WorldMaps {
    fn from_world(world: &mut World) -> Self {
        let dir = read_dir("assets/maps").expect("Maps dir not found");
        let mut maps = WorldMaps(Vec::new());
        for entry in dir {
            let entry = entry.expect("error file in maps dir");
            let path = entry.path();
            let map = MapRepresentation::from_file(path.to_str().unwrap());
            let server: &AssetServer = world.resource::<AssetServer>();
            let handle_image: Handle<Image> = server.load(map.image_path);
            if let Some(music_path) = map.music_path {
                let handle_audio: Handle<AudioSource> = server.load(music_path);
                maps.push((handle_image, Some(handle_audio), map.collisions));
            } else {
                maps.push((handle_image, None, map.collisions));
            }
        }
        maps
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedWorld(0));
        let world = app.world_mut();
        let maps = WorldMaps::from_world(world);
        app.insert_resource(maps);
        app.add_systems(PostStartup, spawn_world)
            .add_systems(FixedUpdate, world_spawn_handle);
    }
}
