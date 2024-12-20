use std::fs::read_dir;

use bevy::prelude::*;
use component::{MapCollision, MapRepresentation};
use systems::{spawn_world, world_spawn_handle};

pub mod component;
mod systems;

pub struct WorldPlugin;

pub type WorldMap = (Handle<Image>, Vec<MapCollision>);

#[derive(Deref, DerefMut, Resource)]
pub struct WorldMaps(pub Vec<WorldMap>);

impl FromWorld for WorldMaps {
    fn from_world(world: &mut World) -> Self {
        let dir = read_dir("assets/maps").expect("Maps dir not found");
        let mut maps = WorldMaps(Vec::new());
        for entry in dir {
            let entry = entry.expect("error file in maps dir");
            let path = entry.path();
            let map = MapRepresentation::from_file(path.to_str().unwrap());
            let handle: Handle<Image> = world.resource::<AssetServer>().load(map.image_path);
            maps.push((handle, map.collisions));
        }
        maps
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        let maps = WorldMaps::from_world(world);
        app.insert_resource(maps);
        app.add_systems(PostStartup, spawn_world)
            .add_systems(FixedPostUpdate, world_spawn_handle);
    }
}
