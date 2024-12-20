use bevy::prelude::*;

#[derive(Resource)]
pub struct MapImg(pub Handle<Image>);

impl MapImg {
    pub fn from_world(world: &mut World, img_path: impl Into<String>) -> Self {
        let handle = world.resource::<AssetServer>().load(img_path.into());
        return MapImg(handle);
    }
}
