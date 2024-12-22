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
    pub fn border_width(mut self, width: f32) -> Self {
        self.border = UiRect::all(Val::Px(width));
        self
    }
    pub fn gap(mut self, value: f32) -> Self {
        self.column_gap = Val::Px(value);
        self.row_gap = Val::Px(value);
        self
    }
    pub fn from_size(width: f32, height: f32) -> Self {
        Self::new().size(width, height)
    }
    pub fn flex_direction(mut self, direction: FlexDirection) -> Self {
        self.flex_direction = direction;
        self
    }
    pub fn padding_all(mut self, all: f32) -> Self {
        self.0.padding = UiRect::all(Val::Px(all));
        self
    }
    pub fn padding_top(mut self, value: f32) -> Self {
        self.0.padding.top = Val::Px(value);
        self
    }
    pub fn padding_bottom(mut self, value: f32) -> Self {
        self.0.padding.bottom = Val::Px(value);
        self
    }
    pub fn padding_x(mut self, value: f32) -> Self {
        self.0.padding.left = Val::Px(value);
        self.0.padding.right = Val::Px(value);
        self
    }
    pub fn padding_left(mut self, value: f32) -> Self {
        self.0.padding.left = Val::Px(value);
        self
    }
    pub fn justify_center(mut self) -> Self {
        self.justify_content = JustifyContent::Center;
        self
    }
    pub fn align_center(mut self) -> Self {
        self.justify_content = JustifyContent::Center;
        self
    }
    pub fn flex(mut self) -> Self {
        self.display = Display::Flex;
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
