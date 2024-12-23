use bevy::prelude::*;

#[derive(Component)]
#[require(GunFireAllowed)]
pub struct Gun {
    pub frequency: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct LastFireTime(Timer);

#[derive(Component, Default)]
pub struct GunFireAllowed;
