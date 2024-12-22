use bevy::prelude::*;

#[derive(Deref, DerefMut)]
pub struct NodeBuilder(Node);

impl NodeBuilder {
    pub fn new() -> NodeBuilder {
        NodeBuilder(Node { ..default() })
    }
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = Val::Px(width);
        self.height = Val::Px(height);
        self
    }
    pub fn from_size(width: f32, height: f32) -> Self {
        Self::new().size(width, height)
    }
    pub fn flex_direction(mut self, direction: FlexDirection) -> Self {
        self.flex_direction = direction;
        self
    }
    pub fn from_flex_direction(direction: FlexDirection) -> Self {
        Self::new().flex_direction(direction)
    }
    pub fn from_min_height(height: f32) -> Self {
        Self::new().min_height(height)
    }
    pub fn min_height(mut self, height: f32) -> Self {
        self.height = Val::Px(height);
        self
    }
    pub fn build(self) -> Node {
        self.0
    }
}
