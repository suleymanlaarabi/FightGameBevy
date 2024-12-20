use bevy::prelude::*;
use component::MapRepresentation;
use resource::MapImg;
use systems::spawn_world;

mod component;
mod resource;
mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        let map = MapRepresentation::from_file("./assets/maps/default_map.json");
        let map_image = MapImg::from_world(app.world_mut(), &map.image_path);
        app.insert_resource(map_image);
        app.insert_resource(map);
        app.add_systems(Startup, spawn_world);
    }
}
