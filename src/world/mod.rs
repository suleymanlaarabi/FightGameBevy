use bevy::prelude::*;
use resource::MapImg;
use systems::spawn_world;

mod resource;
mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        let map_image = MapImg::from_world(app.world_mut());
        app.insert_resource(map_image);
        app.add_systems(Startup, spawn_world);
    }
}
