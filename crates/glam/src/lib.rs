use bevy::prelude::IVec2;
use bevy::prelude::Vec2;

pub trait NegativeYF {
    fn neg_y(&self) -> glam::f32::Vec2;
}
impl NegativeYF for glam::f32::Vec2 {
    fn neg_y(&self) -> glam::f32::Vec2 {
        glam::f32::Vec2::new(self.x, -self.y)
    }
}

pub trait NegativeY {
    fn neg_y(&self) -> Vec2;
}
impl NegativeY for Vec2 {
    fn neg_y(&self) -> Vec2 {
        Vec2::new(self.x, -self.y)
    }
}

pub trait NegativeYI {
    fn neg_y(&self) -> IVec2;
}
impl NegativeYI for IVec2 {
    fn neg_y(&self) -> IVec2 {
        IVec2::new(self.x, -self.y)
    }
}
