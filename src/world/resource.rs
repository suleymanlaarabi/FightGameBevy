use bevy::prelude::*;

#[derive(Resource)]
pub struct MapImg(pub Handle<Image>);

impl FromWorld for MapImg {
    fn from_world(world: &mut World) -> Self {
        let handle = world.resource::<AssetServer>().load("map.png");
        return MapImg(handle);
    }
}
