use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Vec3Wrapper {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vec3> for Vec3Wrapper {
    fn from(vec: Vec3) -> Self {
        Vec3Wrapper {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl Into<Vec3> for Vec3Wrapper {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}
