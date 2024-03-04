use bevy::ecs::entity::Entity;
use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::math::IVec3;
use bevy::math::Rect;
use bevy::math::Vec2;
use bevy::math::Vec3;
use bevy::prelude::Name;

pub trait TopRight {
    fn top_right(&self) -> Vec2;
}
impl TopRight for Rect {
    fn top_right(&self) -> Vec2 {
        Vec2::new(self.max.x, self.min.y)
    }
}

pub trait TopRightI {
    fn top_right(&self) -> IVec2;
}
impl TopRightI for IRect {
    fn top_right(&self) -> IVec2 {
        IVec2::new(self.max.x, self.min.y)
    }
}
