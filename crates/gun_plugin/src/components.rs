use bevy::prelude::*;

#[derive(Component)]
#[require(GunFireAllowed)]
pub struct Gun {
    pub image: Option<Handle<Image>>,
    pub frequency: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct LastFireTime(pub Timer);

#[derive(Component, Default)]
pub struct GunFireAllowed;
