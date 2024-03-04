use bevy::ecs::entity::Entity;
use bevy::math::IRect;
use bevy::math::IVec2;
use bevy::math::IVec3;
use bevy::math::Rect;
use bevy::math::Vec2;
use bevy::math::Vec3;
use bevy::prelude::Name;

pub trait IExpandable {
    fn expand(&self, amount: IVec2) -> IRect;
}
impl IExpandable for IRect {
    fn expand(&self, amount: IVec2) -> IRect {
        IRect::from_center_size(self.center(), self.size() + amount)
    }
}
