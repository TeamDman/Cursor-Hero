use bevy::ecs::entity::Entity;
use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::math::IVec3;
use bevy::math::Rect;
use bevy::math::Vec2;
use bevy::math::Vec3;
use bevy::prelude::Name;

pub trait RectWithHeight {
    fn with_height(&self, height: f32) -> Rect;
}
impl RectWithHeight for Rect {
    fn with_height(&self, height: f32) -> Rect {
        Rect::from_center_size(self.center(), Vec2::new(self.width(), height))
    }
}
