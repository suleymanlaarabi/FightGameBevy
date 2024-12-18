use std::{time::Duration, usize};

use bevy::prelude::*;

pub struct CustomAnimationPlugin;

impl Plugin for CustomAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}

struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct Animation2d {
    pub timer: Timer,
    indices: AnimationIndices,
    disabled: bool,
}

pub fn animate_sprite(time: Res<Time>, mut query: Query<(&mut Animation2d, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut query {
        if !animation.disabled {
            if let Some(atlas) = &mut sprite.texture_atlas {
                animation.timer.tick(time.delta());
                if atlas.index > animation.indices.last || atlas.index < animation.indices.first {
                    atlas.index = animation.indices.first;
                    return;
                }
                if animation.timer.just_finished() {
                    atlas.index = if atlas.index == animation.indices.last {
                        animation.indices.first
                    } else {
                        atlas.index + 1
                    };
                }
            }
        }
    }
}

impl Animation2d {
    pub fn new(duration: f32, first: usize, last: usize) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
            indices: AnimationIndices { first, last },
            disabled: false,
        }
    }
    pub fn set_animation(&mut self, first: usize, last: usize, duration: f32) {
        self.indices.first = first;
        self.indices.last = last;
        self.timer.set_duration(Duration::from_secs_f32(duration));
    }
    pub fn disable(&mut self) -> &mut Self {
        self.disabled = true;
        self
    }
    pub fn enable(&mut self) -> &mut Self {
        self.disabled = false;
        self
    }
    pub fn set_first(&mut self, sprite: &mut Sprite) {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = self.indices.first;
        }
    }
}
