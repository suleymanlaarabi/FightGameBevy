use bevy::prelude::*;

pub struct AppResourcePlugin;

#[derive(Resource)]
pub struct PlayerTileSheet(pub Handle<Image>);

impl FromWorld for PlayerTileSheet {
    fn from_world(world: &mut World) -> Self {
        let handle = world.resource::<AssetServer>().load("adventurer.png");
        return PlayerTileSheet(handle);
    }
}

#[derive(Resource)]
pub struct AdventurerAtlasLayout(pub Handle<TextureAtlasLayout>);

impl FromWorld for AdventurerAtlasLayout {
    fn from_world(world: &mut World) -> Self {
        let atlas = TextureAtlasLayout::from_grid(UVec2::new(50, 37), 7, 11, None, None);
        let handle = world
            .resource_mut::<Assets<TextureAtlasLayout>>()
            .add(atlas);

        return AdventurerAtlasLayout(handle);
    }
}

impl Plugin for AppResourcePlugin {
    fn build(&self, app: &mut App) {
        let player_image = PlayerTileSheet::from_world(app.world_mut());
        let adventurer_atlas = AdventurerAtlasLayout::from_world(app.world_mut());
        app.insert_resource(player_image);
        app.insert_resource(adventurer_atlas);
    }
}
