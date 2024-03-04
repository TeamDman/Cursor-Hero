use bevy::ecs::entity::Entity;
use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::math::IVec3;
use bevy::math::Rect;
use bevy::math::Vec2;
use bevy::math::Vec3;
use bevy::prelude::Name;


pub trait Vec2ToRect {
    fn as_size_of_rect_with_center(&self, center: &Vec2) -> Rect;
}
impl Vec2ToRect for Vec2 {
    fn as_size_of_rect_with_center(&self, center: &Vec2) -> Rect {
        Rect::from_center_size(*center, *self)
    }
}
